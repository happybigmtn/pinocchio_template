#!/bin/bash

# Script to update package.json scripts for new programs
# Usage: ./update-package-scripts.sh [add|remove] [program_name] [category]

set -e

COMMAND="$1"
PROGRAM_NAME="$2"
CATEGORY="${3:-basics}"

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper function to normalize program name for script names (kebab-case)
normalize_script_name() {
    echo "$1" | tr '_' '-'
}

# Helper function to normalize program name for file paths (snake_case)
normalize_file_name() {
    echo "$1" | tr '-' '_'
}

# Function to add scripts for a new program
add_program_scripts() {
    local program_name="$1"
    local category="$2"
    
    local script_name=$(normalize_script_name "$program_name")
    local file_name=$(normalize_file_name "$program_name")
    
    echo -e "${BLUE}Adding package.json scripts for $program_name in category $category...${NC}"
    
    # Read current package.json
    local temp_file=$(mktemp)
    
    # Use Node.js to update package.json properly
    node -e "
        const fs = require('fs');
        const packageJson = JSON.parse(fs.readFileSync('package.json', 'utf8'));
        
        // Check if scripts already exist to avoid duplicates
        const genClientKey = 'gen:client:$script_name';
        const testClientKey = 'test:client:$script_name';
        const genIdlKey = 'gen:idl:$script_name';
        
        let added = false;
        
        // Add new scripts only if they don't exist
        if (!packageJson.scripts[genClientKey]) {
            packageJson.scripts[genClientKey] = 'node scripts/generate-clients.js $program_name';
            added = true;
        }
        if (!packageJson.scripts[testClientKey]) {
            packageJson.scripts[testClientKey] = 'bun test --testFiles $category/$program_name/tests/$script_name.test.ts';
            added = true;
        }
        if (!packageJson.scripts[genIdlKey]) {
            packageJson.scripts[genIdlKey] = 'shank idl --crate-root $category/$file_name --out-dir idl';
            added = true;
        }
        
        if (!added) {
            console.log('Scripts already exist, skipping...');
            process.exit(0);
        }
        
        // Write back to file with proper formatting
        fs.writeFileSync('package.json', JSON.stringify(packageJson, null, 2) + '\n');
    "
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Added scripts for $program_name${NC}"
        echo -e "${GREEN}  - gen:client:$script_name${NC}"
        echo -e "${GREEN}  - test:client:$script_name${NC}"
        echo -e "${GREEN}  - gen:idl:$script_name${NC}"
    else
        echo -e "${RED}Error: Failed to add scripts for $program_name${NC}"
        return 1
    fi
}

# Function to remove scripts for a program
remove_program_scripts() {
    local program_name="$1"
    
    local script_name=$(normalize_script_name "$program_name")
    
    echo -e "${BLUE}Removing package.json scripts for $program_name...${NC}"
    
    # Use Node.js to update package.json properly
    node -e "
        const fs = require('fs');
        const packageJson = JSON.parse(fs.readFileSync('package.json', 'utf8'));
        
        // Remove scripts
        delete packageJson.scripts['gen:client:$script_name'];
        delete packageJson.scripts['test:client:$script_name'];
        delete packageJson.scripts['gen:idl:$script_name'];
        
        // Write back to file with proper formatting
        fs.writeFileSync('package.json', JSON.stringify(packageJson, null, 2) + '\n');
    "
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Removed scripts for $program_name${NC}"
    else
        echo -e "${RED}Error: Failed to remove scripts for $program_name${NC}"
        return 1
    fi
}

# Function to clean all existing scripts (except core ones)
clean_all_scripts() {
    echo -e "${BLUE}Cleaning all program-specific scripts from package.json...${NC}"
    
    # Read existing directories to know what to keep
    local existing_dirs=""
    if [ -d "basics" ]; then
        existing_dirs="$existing_dirs $(find basics -maxdepth 1 -type d -not -name basics | xargs -I {} basename {})"
    fi
    if [ -d "tokens" ]; then
        existing_dirs="$existing_dirs $(find tokens -maxdepth 1 -type d -not -name tokens | xargs -I {} basename {})"
    fi
    
    # Create a temporary file with the existing programs list
    echo "$existing_dirs" > /tmp/existing_programs.txt
    
    # Use Node.js to clean and rebuild scripts
    node -e "
        const fs = require('fs');
        const packageJson = JSON.parse(fs.readFileSync('package.json', 'utf8'));
        
        // Keep core workflow scripts and add program-specific ones
        const newScripts = {
            'new': './scripts/create-program.sh',
            'dep': './scripts/dep-wrapper.sh',
            'gen': './scripts/gen-wrapper.sh'
        };
        
        // Read existing programs from temp file
        const existingProgramsText = fs.readFileSync('/tmp/existing_programs.txt', 'utf8').trim();
        const existingPrograms = existingProgramsText ? existingProgramsText.split(/\s+/).filter(Boolean) : [];
        
        for (const program of existingPrograms) {
            if (!program) continue;
            
            const scriptName = program.replace(/_/g, '-');
            const fileName = program.replace(/-/g, '_');
            
            // Determine category by checking where the program exists
            let category = 'basics';
            if (fs.existsSync('tokens/' + program)) {
                category = 'tokens';
            }
            
            newScripts['gen:client:' + scriptName] = 'node scripts/generate-clients.js ' + program;
            newScripts['test:client:' + scriptName] = 'bun test --testFiles ' + category + '/' + program + '/tests/' + scriptName + '.test.ts';
            newScripts['gen:idl:' + scriptName] = 'shank idl --crate-root ' + category + '/' + fileName + ' --out-dir idl';
        }
        
        packageJson.scripts = newScripts;
        
        // Write back to file with proper formatting
        fs.writeFileSync('package.json', JSON.stringify(packageJson, null, 2) + '\n');
    "
    
    # Clean up temp file
    rm -f /tmp/existing_programs.txt
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Cleaned and rebuilt package.json scripts based on existing directories${NC}"
    else
        echo -e "${RED}Error: Failed to clean package.json scripts${NC}"
        return 1
    fi
}

# Help function
show_help() {
    echo "Usage: $0 [add|remove|clean] [program_name] [category]"
    echo ""
    echo "Manages package.json scripts for Pinocchio programs"
    echo ""
    echo "Commands:"
    echo "  add PROGRAM [CATEGORY]    Add scripts for a new program (default category: basics)"
    echo "  remove PROGRAM           Remove scripts for a program"
    echo "  clean                    Clean all scripts and rebuild from existing directories"
    echo ""
    echo "Arguments:"
    echo "  program_name    Name of the program"
    echo "  category        Program category (basics, tokens, etc.)"
    echo ""
    echo "Examples:"
    echo "  $0 add my-counter basics          # Add scripts for basics/my-counter"
    echo "  $0 add token-mint tokens          # Add scripts for tokens/token-mint"
    echo "  $0 remove my-counter              # Remove scripts for my-counter"
    echo "  $0 clean                          # Clean and rebuild all scripts"
}

# Validate inputs
validate_inputs() {
    case "$COMMAND" in
        add)
            if [ -z "$PROGRAM_NAME" ]; then
                echo -e "${RED}Error: Program name is required for add command${NC}"
                show_help
                exit 1
            fi
            ;;
        remove)
            if [ -z "$PROGRAM_NAME" ]; then
                echo -e "${RED}Error: Program name is required for remove command${NC}"
                show_help
                exit 1
            fi
            ;;
        clean)
            # No additional validation needed
            ;;
        *)
            echo -e "${RED}Error: Invalid command '$COMMAND'${NC}"
            show_help
            exit 1
            ;;
    esac
}

# Main execution
main() {
    if [[ "$1" == "--help" || "$1" == "-h" || -z "$1" ]]; then
        show_help
        exit 0
    fi
    
    validate_inputs
    
    case "$COMMAND" in
        add)
            add_program_scripts "$PROGRAM_NAME" "$CATEGORY"
            ;;
        remove)
            remove_program_scripts "$PROGRAM_NAME"
            ;;
        clean)
            clean_all_scripts
            ;;
    esac
    
    echo -e "${GREEN}✅ Package.json update completed!${NC}"
}

# Run main function
main "$@"
