#!/bin/bash

# Script to clean up duplicate workspace members in Cargo.toml
# This fixes the issue where "basics/*" and "templates/*" were added multiple times

set -e

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Cleaning up duplicate workspace members in Cargo.toml...${NC}"

# Check if Cargo.toml exists
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}Error: Cargo.toml not found${NC}"
    exit 1
fi

# Create a backup
cp Cargo.toml Cargo.toml.backup
echo -e "${GREEN}✓ Created backup: Cargo.toml.backup${NC}"

# Read the current workspace members and remove duplicates
# This script will reconstruct the members array with unique entries only
python3 -c "
import re

# Read the file
with open('Cargo.toml', 'r') as f:
    content = f.read()

# Extract the members section
members_pattern = r'members = \[(.*?)\]'
match = re.search(members_pattern, content, re.DOTALL)

if match:
    members_content = match.group(1)
    
    # Extract all member entries
    member_entries = re.findall(r'\"([^\"]+)\"', members_content)
    
    # Remove duplicates while preserving order
    unique_members = []
    seen = set()
    
    for member in member_entries:
        if member not in seen:
            unique_members.append(member)
            seen.add(member)
    
    # Remove templates/* as it should not be a workspace member
    unique_members = [m for m in unique_members if not m.startswith('templates/')]
    
    # Reconstruct the members array
    new_members_content = '[\n'
    for member in unique_members:
        new_members_content += f'  \"{member}\",\n'
    new_members_content += ']'
    
    # Replace in content
    new_content = re.sub(members_pattern, f'members = {new_members_content}', content, flags=re.DOTALL)
    
    # Write back
    with open('Cargo.toml', 'w') as f:
        f.write(new_content)
    
    print(f'Cleaned up workspace members. Unique members: {unique_members}')
else:
    print('Could not find members section in Cargo.toml')
"

echo -e "${GREEN}✓ Cleaned up duplicate workspace members${NC}"
echo -e "${YELLOW}Note: Removed 'templates/*' as templates should not be workspace members${NC}"

# Show the difference
echo -e "${BLUE}Changes made:${NC}"
diff -u Cargo.toml.backup Cargo.toml || true

echo -e "${GREEN}✓ Cleanup completed successfully!${NC}"
echo -e "${YELLOW}The backup file Cargo.toml.backup can be deleted if you're satisfied with the changes.${NC}"
