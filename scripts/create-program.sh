#!/bin/bash

# Script to create new Pinocchio programs from template
# Usage: ./create-program.sh [program_name] [--category=basics|tokens|compression|oracles] [--template=template_name]

set -e

# Default values
PROGRAM_NAME="$1"
CATEGORY="basics"
TEMPLATE_NAME="counter"
TEMPLATE_DIR=""

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Help function
show_help() {
    echo "Usage: $0 [program_name] [--category=basics|tokens|compression|oracles] [--template=template_name]"
    echo ""
    echo "Creates a new Pinocchio program from template"
    echo ""
    echo "Arguments:"
    echo "  program_name         Name of the new program (required)"
    echo "  --category=CATEGORY  Category for the program (default: basics)"
    echo "  --template=TEMPLATE  Template to use (default: counter)"
    echo ""
    echo "Examples:"
    echo "  $0 my_program                              # Create basics/my_program from counter template"
    echo "  $0 token_mint --category=tokens            # Create tokens/token_mint from counter template"
    echo "  $0 my_counter --template=account-data      # Create basics/my_counter from account-data template"
    echo "  $0 my_program --category=tokens --template=account-data  # Create tokens/my_program from account-data template"
    echo ""
    echo "Available templates:"
    echo "  - counter (default): Basic counter program template"
    echo "  - account-data: Account data management template"
    echo ""
    echo "The script will:"
    echo "  1. Create program directory with Rust source code"
    echo "  2. Update Cargo.toml files"
    echo "  3. Generate package.json scripts"
    echo "  4. Create test files"
    echo "  5. Set up deployment configuration"
}

# Validate inputs
validate_inputs() {
    if [ -z "$PROGRAM_NAME" ]; then
        echo -e "${RED}Error: Program name is required${NC}"
        show_help
        exit 1
    fi

    # Validate program name format
    if [[ ! "$PROGRAM_NAME" =~ ^[a-z0-9_-]+$ ]]; then
        echo -e "${RED}Error: Program name must contain only lowercase letters, numbers, hyphens, and underscores${NC}"
        exit 1
    fi

    # Validate category
    case $CATEGORY in
        basics|tokens|compression|oracles)
            ;;
        *)
            echo -e "${RED}Error: Invalid category '$CATEGORY'. Use basics, tokens, compression, or oracles${NC}"
            exit 1
            ;;
    esac

    # Set template directory based on template name
    case $TEMPLATE_NAME in
        counter)
            TEMPLATE_DIR="templates/counter"
            ;;
        account-data)
            TEMPLATE_DIR="templates/account-data"
            ;;
        *)
            echo -e "${RED}Error: Unknown template '$TEMPLATE_NAME'. Available templates: counter, account-data${NC}"
            exit 1
            ;;
    esac

    # Check if program already exists
    if [ -d "$CATEGORY/$PROGRAM_NAME" ]; then
        echo -e "${RED}Error: Program $CATEGORY/$PROGRAM_NAME already exists${NC}"
        exit 1
    fi

    # Check if template exists
    if [ ! -d "$TEMPLATE_DIR" ]; then
        echo -e "${RED}Error: Template directory $TEMPLATE_DIR does not exist${NC}"
        exit 1
    fi
}

# Create program directory structure
create_program_structure() {
    echo -e "${BLUE}Creating program structure for $PROGRAM_NAME...${NC}"
    
    local target_dir="$CATEGORY/$PROGRAM_NAME"
    mkdir -p "$target_dir"
    
    # Copy template files excluding tests directory
    cp -r "$TEMPLATE_DIR"/* "$target_dir/"
    
    # Remove the copied tests directory to avoid unnecessary test files
    rm -rf "$target_dir/tests"
    
    # Create directory structure
    mkdir -p "$target_dir/src/instructions"
    mkdir -p "$target_dir/src/state"
    mkdir -p "$target_dir/tests"
    
    echo -e "${GREEN}✓ Program structure created${NC}"
}

# Update Cargo.toml files
update_cargo_toml() {
    echo -e "${BLUE}Updating Cargo.toml files...${NC}"
    
    local target_dir="$CATEGORY/$PROGRAM_NAME"
    local program_name_snake=$(echo "$PROGRAM_NAME" | tr '-' '_')
    
    # Update program Cargo.toml - replace any template name patterns with the new program name
    if [ -f "$target_dir/Cargo.toml" ]; then
        # Replace common template name patterns
        sed -i "s/name = \"account-data-template\"/name = \"$PROGRAM_NAME\"/g" "$target_dir/Cargo.toml"
        sed -i "s/name = \"counter-template\"/name = \"$PROGRAM_NAME\"/g" "$target_dir/Cargo.toml"
        sed -i "s/name = \"account-data\"/name = \"$PROGRAM_NAME\"/g" "$target_dir/Cargo.toml"
        sed -i "s/name = \"counter\"/name = \"$PROGRAM_NAME\"/g" "$target_dir/Cargo.toml"
        
        # Replace with snake_case version if needed
        sed -i "s/name = \"account_data_template\"/name = \"$program_name_snake\"/g" "$target_dir/Cargo.toml"
        sed -i "s/name = \"counter_template\"/name = \"$program_name_snake\"/g" "$target_dir/Cargo.toml"
        sed -i "s/name = \"account_data\"/name = \"$program_name_snake\"/g" "$target_dir/Cargo.toml"
        
        echo -e "${GREEN}✓ Updated Cargo.toml package name to $PROGRAM_NAME${NC}"
    fi
    
    # Update workspace Cargo.toml
    if [ -f "Cargo.toml" ]; then
        # Check if we need to add explicit member or if wildcard already covers it
        local category_wildcard="\"$CATEGORY/*\""
        
        # If category wildcard doesn't exist, add it
        if ! grep -q "$category_wildcard" Cargo.toml; then
            echo -e "${YELLOW}Adding $CATEGORY category to workspace members...${NC}"
            # Add the category wildcard to members array
            sed -i "/members = \[/,/\]/ s/\]/  \"$CATEGORY\/*\",\n\]/" Cargo.toml
        else
            echo -e "${GREEN}✓ $CATEGORY category already in workspace members${NC}"
        fi
    fi
    
    echo -e "${GREEN}✓ Cargo.toml files updated${NC}"
}

# Update Rust source files
update_rust_source() {
    echo -e "${BLUE}Updating Rust source files...${NC}"
    
    local target_dir="$CATEGORY/$PROGRAM_NAME"
    local program_name_snake=$(echo "$PROGRAM_NAME" | tr '-' '_')
    local program_name_upper=$(echo "$PROGRAM_NAME" | tr '[:lower:]' '[:upper:]' | tr '-' '_')
    
    # Generate a new program ID placeholder
    local new_program_id="11111111111111111111111111111111"
    
    # Determine the original template name to replace
    local template_name_snake
    case $TEMPLATE_NAME in
        counter)
            template_name_snake="counter"
            ;;
        account-data)
            template_name_snake="account_data"
            ;;
        *)
            template_name_snake="counter"  # fallback
            ;;
    esac
    
    local template_name_upper=$(echo "$template_name_snake" | tr '[:lower:]' '[:upper:]')
    
    # Update lib.rs
    if [ -f "$target_dir/src/lib.rs" ]; then
        sed -i "s/$template_name_snake/$program_name_snake/g" "$target_dir/src/lib.rs"
        sed -i "s/$template_name_upper/$program_name_upper/g" "$target_dir/src/lib.rs"
        sed -i "s/pinocchio_pubkey::declare_id!(\"[^\"]*\")/pinocchio_pubkey::declare_id!(\"$new_program_id\")/g" "$target_dir/src/lib.rs"
    fi
    
    # Update other Rust files
    find "$target_dir/src" -name "*.rs" -exec sed -i "s/$template_name_snake/$program_name_snake/g" {} \;
    find "$target_dir/src" -name "*.rs" -exec sed -i "s/$template_name_upper/$program_name_upper/g" {} \;
    
    # Also update test files and utilities
    find "$target_dir/tests" -name "*.rs" -exec sed -i "s/$template_name_snake/$program_name_snake/g" {} \; 2>/dev/null || true
    find "$target_dir/tests" -name "*.rs" -exec sed -i "s/$template_name_upper/$program_name_upper/g" {} \; 2>/dev/null || true
    
    # Update template crate names in all test files
    find "$target_dir/tests" -name "*.rs" -exec sed -i "s/counter_template/$program_name_snake/g" {} \; 2>/dev/null || true
    find "$target_dir/tests" -name "*.rs" -exec sed -i "s/account_data_template/$program_name_snake/g" {} \; 2>/dev/null || true
    find "$target_dir/tests" -name "*.rs" -exec sed -i "s/counter-template/$program_name_dash/g" {} \; 2>/dev/null || true
    find "$target_dir/tests" -name "*.rs" -exec sed -i "s/account-data-template/$program_name_dash/g" {} \; 2>/dev/null || true
    
    # Update TypeScript files
    find "$target_dir/tests" -name "*.ts" -exec sed -i "s/counter_template/$program_name_snake/g" {} \; 2>/dev/null || true
    find "$target_dir/tests" -name "*.ts" -exec sed -i "s/account_data_template/$program_name_snake/g" {} \; 2>/dev/null || true
    find "$target_dir/tests" -name "*.ts" -exec sed -i "s/counter-template/$program_name_dash/g" {} \; 2>/dev/null || true
    find "$target_dir/tests" -name "*.ts" -exec sed -i "s/account-data-template/$program_name_dash/g" {} \; 2>/dev/null || true
    
    echo -e "${GREEN}✓ Rust source files updated${NC}"
}

# Update package.json scripts
update_package_json() {
    echo -e "${BLUE}Updating package.json scripts...${NC}"
    
    # Call the package script updater
    chmod +x "./scripts/update-package-scripts.sh"
    "./scripts/update-package-scripts.sh" add "$PROGRAM_NAME" "$CATEGORY"
    
    echo -e "${GREEN}✓ package.json scripts updated${NC}"
}

# Create test files
create_test_files() {
    echo -e "${BLUE}Creating test files...${NC}"
    
    local target_dir="$CATEGORY/$PROGRAM_NAME"
    local program_name_dash=$(echo "$PROGRAM_NAME" | tr '_' '-')
    local program_name_snake=$(echo "$PROGRAM_NAME" | tr '-' '_')
    
    # Create tests directory if it doesn't exist
    mkdir -p "$target_dir/tests"
    
    # Create minimal skeleton Rust test file that won't interfere with compilation
    cat > "$target_dir/tests/${program_name_snake}.rs" << EOF
// Test file for ${program_name_snake}
// Add your Mollusk SVM tests here

#[cfg(test)]
mod tests {
    use ${program_name_snake}::ID;
    use solana_sdk::pubkey::Pubkey;

    pub const PROGRAM_ID: Pubkey = Pubkey::new_from_array(ID);

    #[test]
    fn test_program_id() {
        // Basic test that verifies program ID is set correctly
        assert_ne!(PROGRAM_ID, Pubkey::default());
    }
}
EOF

    # Create minimal skeleton TypeScript test file that won't interfere with compilation
    cat > "$target_dir/tests/${program_name_dash}.test.ts" << EOF
// Test file for ${program_name_dash}
// Add your Solana Kite tests here

import { expect, test } from 'bun:test';

test('${program_name_dash}:basic', async () => {
  // Basic test that always passes - replace with actual tests
  expect(true).toBe(true);
});
EOF

    echo -e "${GREEN}✓ Created minimal skeleton test files${NC}"
}

# Create deployment configuration
create_deployment_config() {
    echo -e "${BLUE}Creating deployment configuration...${NC}"
    
    local target_dir="$CATEGORY/$PROGRAM_NAME"
    
    # Create deploy script for this specific program
    cat > "$target_dir/deploy.sh" << 'EOF'
#!/bin/bash

# Program-specific deployment script
# This is a convenience script that calls the main deploy.sh

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$ROOT_DIR"

# Extract program name from directory structure
PROGRAM_NAME=$(basename "$SCRIPT_DIR")

# Call main deployment script
exec "./scripts/deploy.sh" "$PROGRAM_NAME" "$@"
EOF
    
    chmod +x "$target_dir/deploy.sh"
    
    echo -e "${GREEN}✓ Deployment configuration created${NC}"
}

# Create README for the new program
create_readme() {
    echo -e "${BLUE}Creating README...${NC}"
    
    local target_dir="$CATEGORY/$PROGRAM_NAME"
    local program_name_dash=$(echo "$PROGRAM_NAME" | tr '_' '-')
    local program_name_title=$(echo "$PROGRAM_NAME" | tr '_-' ' ' | sed 's/\b\w/\U&/g')
    
    cat > "$target_dir/README.md" << EOF
# ${program_name_title}

A Solana program built with Pinocchio.

## Description

TODO: Add description of what this program does.

## Usage

### Building

\`\`\`bash
cargo build-sbf --manifest-path $CATEGORY/$PROGRAM_NAME/Cargo.toml
\`\`\`

### Deployment

\`\`\`bash
# Deploy to devnet
./deploy.sh $PROGRAM_NAME

# Deploy to testnet
./deploy.sh $PROGRAM_NAME --network=testnet

# Deploy to mainnet
./deploy.sh $PROGRAM_NAME --network=mainnet
\`\`\`

### Generate Client

\`\`\`bash
# Generate IDL
npm run gen:idl:$program_name_dash

# Generate TypeScript client
npm run gen:client:$program_name_dash
\`\`\`

### Testing

\`\`\`bash
# Run tests
npm run test:client:$program_name_dash
\`\`\`

## Program Structure

- \`src/lib.rs\` - Main program entry point
- \`src/processor.rs\` - Instruction processing logic
- \`src/instructions/\` - Instruction definitions
- \`src/state/\` - Account state definitions
- \`src/constants.rs\` - Program constants
- \`tests/\` - Test files

## Development

1. Modify the Rust source code in \`src/\`
2. Build and deploy: \`./deploy.sh $PROGRAM_NAME\`
3. Generate client: \`npm run gen:client:$program_name_dash\`
4. Run tests: \`npm run test:client:$program_name_dash\`

## Notes

- This program was created from the Pinocchio template
- Program ID will be updated after first deployment
- Remember to commit changes after successful deployment
EOF
    
    echo -e "${GREEN}✓ README created${NC}"
}

# Display summary
show_summary() {
    echo ""
    echo -e "${GREEN}🎉 Program Creation Summary${NC}"
    echo -e "${GREEN}===========================${NC}"
    echo -e "${GREEN}Program Name: $PROGRAM_NAME${NC}"
    echo -e "${GREEN}Category: $CATEGORY${NC}"
    echo -e "${GREEN}Location: $CATEGORY/$PROGRAM_NAME${NC}"
    echo ""
    echo -e "${YELLOW}Next steps:${NC}"
    echo -e "${YELLOW}1. Review the generated code in $CATEGORY/$PROGRAM_NAME/src/${NC}"
    echo -e "${YELLOW}2. Modify the program logic as needed${NC}"
    echo -e "${YELLOW}3. Deploy the program: ./deploy.sh $PROGRAM_NAME${NC}"
    echo -e "${YELLOW}4. Generate client: npm run gen:client:$(echo $PROGRAM_NAME | tr '_' '-')${NC}"
    echo -e "${YELLOW}5. Run tests: npm run test:client:$(echo $PROGRAM_NAME | tr '_' '-')${NC}"
    echo ""
    echo -e "${GREEN}Files created:${NC}"
    find "$CATEGORY/$PROGRAM_NAME" -type f | sort
}

# Main execution
main() {
    echo -e "${GREEN}🚀 Creating new Pinocchio program: $PROGRAM_NAME${NC}"
    echo -e "${GREEN}📋 Using template: $TEMPLATE_NAME${NC}"
    echo -e "${GREEN}📁 Target category: $CATEGORY${NC}"
    echo ""
    
    validate_inputs
    create_program_structure
    update_cargo_toml
    update_rust_source
    update_package_json
    create_test_files
    create_deployment_config
    create_readme
    show_summary
    
    echo -e "${GREEN}✅ Program creation completed successfully!${NC}"
}

# Parse command line arguments
for arg in "$@"; do
    case $arg in
        --category=*)
            CATEGORY="${arg#*=}"
            ;;
        --template=*)
            TEMPLATE_NAME="${arg#*=}"
            ;;
        --help|-h)
            show_help
            exit 0
            ;;
    esac
done

# Check for help flag
if [[ "$1" == "--help" || "$1" == "-h" ]]; then
    show_help
    exit 0
fi

# Run main function
main
