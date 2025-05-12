#!/bin/bash

cd data_engine; bash build.sh; cd -;
cp ~/CyberCraft/src/data_engine/target/release/data_engine ~/.cybercraft/data_engine
chmod +x ~/.cybercraft/data_engine
cp ~/CyberCraft/src/data_engine/target/release/data_engine ./bin/


