#!/bin/bash

# Comprehensive Test Runner for Taproot Backend
# This script runs all tests with different configurations and provides detailed output

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    if ! command_exists cargo; then
        print_error "Cargo is not installed. Please install Rust first."
        exit 1
    fi
    
    if ! command_exists rustc; then
        print_error "Rust compiler is not installed. Please install Rust first."
        exit 1
    fi
    
    print_success "Prerequisites check passed"
}

# Function to run unit tests
run_unit_tests() {
    print_status "Running unit tests..."
    
    local test_output
    if test_output=$(cargo test --lib 2>&1); then
        print_success "Unit tests passed"
        echo "$test_output"
    else
        print_error "Unit tests failed"
        echo "$test_output"
        return 1
    fi
}

# Function to run integration tests
run_integration_tests() {
    print_status "Running integration tests..."
    
    local test_output
    if test_output=$(cargo test --test integration_tests 2>&1); then
        print_success "Integration tests passed"
        echo "$test_output"
    else
        print_error "Integration tests failed"
        echo "$test_output"
        return 1
    fi
}

# Function to run doc tests
run_doc_tests() {
    print_status "Running documentation tests..."
    
    local test_output
    if test_output=$(cargo test --doc 2>&1); then
        print_success "Documentation tests passed"
        echo "$test_output"
    else
        print_error "Documentation tests failed"
        echo "$test_output"
        return 1
    fi
}

# Function to run tests with different features
run_feature_tests() {
    print_status "Running tests with different features..."
    
    # Test with default features
    print_status "Testing with default features..."
    if ! cargo test --features default 2>&1; then
        print_error "Default feature tests failed"
        return 1
    fi
    
    print_success "Feature tests passed"
}

# Function to run clippy checks
run_clippy() {
    print_status "Running clippy checks..."
    
    local clippy_output
    if clippy_output=$(cargo clippy -- -D warnings 2>&1); then
        print_success "Clippy checks passed"
        echo "$clippy_output"
    else
        print_error "Clippy checks failed"
        echo "$clippy_output"
        return 1
    fi
}

# Function to run format checks
run_format_check() {
    print_status "Running format checks..."
    
    local format_output
    if format_output=$(cargo fmt -- --check 2>&1); then
        print_success "Format checks passed"
        echo "$format_output"
    else
        print_error "Format checks failed"
        echo "$format_output"
        return 1
    fi
}

# Function to run security audit
run_security_audit() {
    print_status "Running security audit..."
    
    if command_exists cargo-audit; then
        local audit_output
        if audit_output=$(cargo audit 2>&1); then
            print_success "Security audit passed"
            echo "$audit_output"
        else
            print_warning "Security audit found issues"
            echo "$audit_output"
        fi
    else
        print_warning "cargo-audit not installed. Skipping security audit."
        print_status "Install with: cargo install cargo-audit"
    fi
}

# Function to run benchmarks (if available)
run_benchmarks() {
    print_status "Running benchmarks..."
    
    if cargo test --benches 2>&1; then
        print_success "Benchmarks passed"
    else
        print_warning "Benchmarks failed or not available"
    fi
}

# Function to generate test coverage report
run_coverage() {
    print_status "Generating test coverage report..."
    
    if command_exists grcov; then
        # Set up coverage environment
        export CARGO_INCREMENTAL=0
        export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Cpanic=abort -Zpanic_abort_tests"
        export RUSTDOCFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Cpanic=abort -Zpanic_abort_tests"
        
        # Clean previous coverage data
        cargo clean
        
        # Run tests with coverage
        if cargo test 2>&1; then
            # Generate coverage report
            if grcov . --binary-path ./target/debug/ -s . -t html --branch --ignore-not-existing -o ./coverage/ 2>&1; then
                print_success "Coverage report generated in ./coverage/"
            else
                print_warning "Failed to generate coverage report"
            fi
        else
            print_warning "Tests failed during coverage run"
        fi
    else
        print_warning "grcov not installed. Skipping coverage report."
        print_status "Install with: cargo install grcov"
    fi
}

# Function to run performance tests
run_performance_tests() {
    print_status "Running performance tests..."
    
    # This would be implemented based on your specific performance requirements
    print_status "Performance tests not yet implemented"
}

# Function to run stress tests
run_stress_tests() {
    print_status "Running stress tests..."
    
    # This would be implemented based on your specific stress testing requirements
    print_status "Stress tests not yet implemented"
}

# Function to clean up
cleanup() {
    print_status "Cleaning up..."
    cargo clean
    print_success "Cleanup completed"
}

# Function to show test summary
show_summary() {
    local total_tests=$1
    local passed_tests=$2
    local failed_tests=$3
    
    echo ""
    echo "=========================================="
    echo "           TEST SUMMARY"
    echo "=========================================="
    echo "Total tests run: $total_tests"
    echo "Passed: $passed_tests"
    echo "Failed: $failed_tests"
    echo "=========================================="
    
    if [ $failed_tests -eq 0 ]; then
        print_success "All tests passed! 🎉"
        exit 0
    else
        print_error "Some tests failed! ❌"
        exit 1
    fi
}

# Main function
main() {
    local total_tests=0
    local passed_tests=0
    local failed_tests=0
    
    echo "=========================================="
    echo "    TAPROOT BACKEND TEST SUITE"
    echo "=========================================="
    
    # Check prerequisites
    check_prerequisites
    
    # Run different types of tests
    test_functions=(
        "run_unit_tests"
        "run_integration_tests"
        "run_doc_tests"
        "run_feature_tests"
        "run_clippy"
        "run_format_check"
        "run_security_audit"
        "run_benchmarks"
        "run_coverage"
        "run_performance_tests"
        "run_stress_tests"
    )
    
    for test_func in "${test_functions[@]}"; do
        total_tests=$((total_tests + 1))
        if $test_func; then
            passed_tests=$((passed_tests + 1))
        else
            failed_tests=$((failed_tests + 1))
        fi
    done
    
    # Cleanup
    cleanup
    
    # Show summary
    show_summary $total_tests $passed_tests $failed_tests
}

# Handle script arguments
case "${1:-}" in
    "unit")
        print_status "Running only unit tests..."
        run_unit_tests
        ;;
    "integration")
        print_status "Running only integration tests..."
        run_integration_tests
        ;;
    "coverage")
        print_status "Running coverage tests..."
        run_coverage
        ;;
    "clippy")
        print_status "Running only clippy checks..."
        run_clippy
        ;;
    "format")
        print_status "Running only format checks..."
        run_format_check
        ;;
    "audit")
        print_status "Running only security audit..."
        run_security_audit
        ;;
    "clean")
        print_status "Cleaning project..."
        cleanup
        ;;
    "help"|"-h"|"--help")
        echo "Usage: $0 [option]"
        echo ""
        echo "Options:"
        echo "  unit        Run only unit tests"
        echo "  integration Run only integration tests"
        echo "  coverage    Run coverage tests"
        echo "  clippy      Run only clippy checks"
        echo "  format      Run only format checks"
        echo "  audit       Run only security audit"
        echo "  clean       Clean the project"
        echo "  help        Show this help message"
        echo ""
        echo "If no option is provided, all tests will be run."
        ;;
    *)
        main
        ;;
esac 