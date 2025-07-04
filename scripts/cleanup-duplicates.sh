#!/bin/bash

# Script to clean up duplicate workspace members and package scripts
# Usage: ./cleanup-duplicates.sh

set -e

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🧹 Cleaning up duplicates...${NC}"

# Clean up Cargo.toml duplicates
cleanup_cargo_toml() {
    echo -e "${BLUE}Removing duplicate workspace members from Cargo.toml...${NC}"
    
    if [ ! -f "Cargo.toml" ]; then
        echo -e "${RED}Error: Cargo.toml not found${NC}"
        return 1
    fi
    
    # Use Python to clean duplicates while preserving order
    python3 << 'EOF'
import re
import sys

try:
    with open('Cargo.toml', 'r') as f:
        content = f.read()
    
    # Find the members array
    members_pattern = r'members = \[(.*?)\]'
    match = re.search(members_pattern, content, re.DOTALL)
    
    if not match:
        print("No members array found in Cargo.toml")
        sys.exit(0)
    
    members_content = match.group(1)
    
    # Extract individual members
    member_pattern = r'"([^"]+)"'
    members = re.findall(member_pattern, members_content)
    
    # Remove duplicates while preserving order
    seen = set()
    unique_members = []
    for member in members:
        if member not in seen:
            seen.add(member)
            unique_members.append(member)
    
    # Check if duplicates were found
    if len(members) != len(unique_members):
        print(f"Found {len(members) - len(unique_members)} duplicate(s)")
        
        # Reconstruct the members array
        new_members_content = '[\n'
        for member in unique_members:
            new_members_content += f'  "{member}", \n'
        new_members_content += ']'
        
        # Replace in the content
        new_content = re.sub(members_pattern, f'members = {new_members_content}', content, flags=re.DOTALL)
        
        # Write back to file
        with open('Cargo.toml', 'w') as f:
            f.write(new_content)
        
        print("✓ Removed duplicates from Cargo.toml")
    else:
        print("✓ No duplicates found in Cargo.toml")

except Exception as e:
    print(f"Error: {e}")
    sys.exit(1)
EOF
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Cargo.toml cleanup completed${NC}"
    else
        echo -e "${RED}✗ Failed to clean up Cargo.toml${NC}"
        return 1
    fi
}

# Clean up package.json scripts
cleanup_package_json() {
    echo -e "${BLUE}Cleaning up package.json scripts...${NC}"
    
    if [ -x "./scripts/update-package-scripts.sh" ]; then
        ./scripts/update-package-scripts.sh clean
        echo -e "${GREEN}✓ package.json scripts cleaned${NC}"
    else
        echo -e "${YELLOW}⚠ update-package-scripts.sh not found or not executable${NC}"
    fi
}

# Validate workspace after cleanup
validate_workspace() {
    echo -e "${BLUE}Validating workspace...${NC}"
    
    if cargo check --quiet; then
        echo -e "${GREEN}✓ Workspace validation passed${NC}"
    else
        echo -e "${RED}✗ Workspace validation failed${NC}"
        return 1
    fi
}

# Show summary
show_summary() {
    echo ""
    echo -e "${GREEN}🎉 Cleanup Summary${NC}"
    echo -e "${GREEN}==================${NC}"
    echo -e "${GREEN}✓ Removed duplicate workspace members${NC}"
    echo -e "${GREEN}✓ Cleaned package.json scripts${NC}"
    echo -e "${GREEN}✓ Validated workspace integrity${NC}"
    echo ""
    echo -e "${YELLOW}Current workspace members:${NC}"
    if [ -f "Cargo.toml" ]; then
        grep -A 10 "members = \[" Cargo.toml | grep "\"" | sed 's/^[[:space:]]*/  /'
    fi
}

# Main execution
main() {
    echo -e "${GREEN}🧹 Starting duplicate cleanup...${NC}"
    echo ""
    
    cleanup_cargo_toml
    cleanup_package_json
    validate_workspace
    show_summary
    
    echo -e "${GREEN}✅ Cleanup completed successfully!${NC}"
}

# Help function
show_help() {
    echo "Usage: $0"
    echo ""
    echo "Cleans up duplicate workspace members and package scripts"
    echo ""
    echo "This script will:"
    echo "  1. Remove duplicate members from Cargo.toml"
    echo "  2. Clean and rebuild package.json scripts"
    echo "  3. Validate workspace integrity"
    echo ""
    echo "Examples:"
    echo "  $0              # Clean up duplicates"
    echo "  $0 --help       # Show this help"
}

# Check for help flag
if [[ "$1" == "--help" || "$1" == "-h" ]]; then
    show_help
    exit 0
fi

# Run main function
main
