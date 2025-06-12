#!/bin/bash

# Script to generate random transactions in CSV format to stdout
# Usage: ./generate_rand_tx.sh [number_of_transactions]
# Example: ./generate_rand_tx.sh 50 > output.csv

set -e

# Default number of transactions
NUM_TRANSACTIONS=${1:-100}

# Output CSV header
echo "type,client,tx,amount"

# Generate random transactions
for ((i=1; i<=NUM_TRANSACTIONS; i++)); do
    # Random transaction type (deposit, withdrawal, dispute, resolve, chargeback)
    TYPES=("deposit" "withdrawal" "dispute" "resolve" "chargeback")
    TYPE=${TYPES[$((RANDOM % ${#TYPES[@]}))]}
    
    # Random client ID (0 - 50)
    CLIENT=$((RANDOM % 50))
    
    # Sequential transaction ID
    TX_ID=$i
    
    # Random amount (0.01 to 9999.99) - only for deposit and withdrawal
    if [[ "$TYPE" == "deposit" || "$TYPE" == "withdrawal" ]]; then
        # Generate amount with 2 decimal places (1 to 999999 cents, then divide by 100)
        AMOUNT_CENTS=$((RANDOM % 999999 + 1))
        AMOUNT=$(printf "%.2f" $(echo "scale=2; $AMOUNT_CENTS / 100" | bc))
        echo "$TYPE,$CLIENT,$TX_ID,$AMOUNT"
    else
        # For dispute, resolve, chargeback - reference existing transaction
        REF_TX=$((RANDOM % i))
        echo "$TYPE,$CLIENT,$REF_TX,"
    fi
done
