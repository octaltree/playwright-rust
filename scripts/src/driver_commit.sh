#!/bin/bash
# find commit corresponding to the driver
# use in latest main of playwright

function check(){
  [ `same_protocol` -ne 0 ] || [ `same_api` -ne 0 ]
}

function same_protocol(){
  diff ~/.cache/ms-playwright/playwright-rust/driver/package/protocol.yml packages/playwright-core/src/protocol/protocol.yml| wc -l
}

function same_api(){
  API_JSON_MODE=1 node utils/doclint/generateApiJson.js > ./output/api.json
  diff ~/.cache/ms-playwright/playwright-rust/driver/package/api.json output/api.json| wc -l
}

while check; do
  git checkout HEAD~
done
