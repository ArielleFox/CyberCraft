#!/bin/bash

cd data_engine; cargo build; cd -;
cp ~/CyberCraft/src/data_engine/target/debug/data_engine ~/.cybercraft/data_engine
chmod +x ~/.cybercraft/data_engine
cp ~/CyberCraft/src/data_engine/target/debug/data_engine ./bin/


