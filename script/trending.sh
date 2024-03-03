#!/bin/bash

API_KEY="702e27e0-125f-4a90-8ff4-14259a73fcf8"

curl -H "X-CMC_PRO_API_KEY: $API_KEY" -H "Accept: application/json" -d "start=1&limit=100&convert=USD" -G https://pro-api.coinmarketcap.com/v1/cryptocurrency/trending/latest

