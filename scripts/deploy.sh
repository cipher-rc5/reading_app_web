#!/usr/bin/env bash

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo -e "${BLUE}================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}================================${NC}"
}

print_header "GitHub Pages Deployment"

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    print_error "Not in a git repository"
    exit 1
fi

# Check for uncommitted changes
if ! git diff-index --quiet HEAD -- 2>/dev/null; then
    print_warn "You have uncommitted changes"
    read -p "Continue anyway? (yes/no): " -r
    if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
        print_info "Deployment cancelled"
        exit 0
    fi
fi

# Get current branch
CURRENT_BRANCH=$(git branch --show-current)
REPO_URL=$(git config --get remote.origin.url)
REPO_NAME=$(basename "$(git rev-parse --show-toplevel)")

print_info "Current branch: $CURRENT_BRANCH"
print_info "Repository: $REPO_NAME"

# Confirm deployment
if [[ "$CURRENT_BRANCH" == "main" ]] || [[ "$CURRENT_BRANCH" == "master" ]]; then
    print_warn "Deploying from $CURRENT_BRANCH branch"
    read -p "This will trigger GitHub Actions deployment. Continue? (yes/no): " -r
    if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
        print_info "Deployment cancelled"
        exit 0
    fi
else
    print_warn "You're on branch '$CURRENT_BRANCH', not main/master"
    print_info "GitHub Pages typically deploys from main branch"
    read -p "Push to current branch anyway? (yes/no): " -r
    if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
        print_info "Deployment cancelled"
        exit 0
    fi
fi

print_info "Pushing to remote..."
git push origin "$CURRENT_BRANCH"

print_header "Deployment Initiated"
print_info "GitHub Actions will build and deploy your app"
print_info "Monitor progress at: ${REPO_URL%.git}/actions"
print_info ""
print_info "Once deployed, your app will be available at:"
print_info "https://$(git config --get remote.origin.url | sed 's/.*github.com[:/]\(.*\)\.git/\1/' | cut -d'/' -f1).github.io/$REPO_NAME/"
print_info ""
print_warn "Note: First deployment may take a few minutes"
print_warn "Make sure GitHub Pages is enabled in your repository settings:"
print_warn "  Settings > Pages > Source > Deploy from a branch > gh-pages > / (root)"
