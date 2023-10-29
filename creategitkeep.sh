#!/bin/bash

# ディレクトリ構造の作成と.gitkeepファイルの配置
find ./src -type d -path '*/testcases/*/in' -exec bash -c 'echo -n > "$0"/.gitkeep' {} \;
find ./src -type d -path '*/testcases/*/out' -exec bash -c 'echo -n > "$0"/.gitkeep' {} \;
