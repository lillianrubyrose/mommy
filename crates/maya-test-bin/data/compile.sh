#!/bin/sh
javac -d out/ --module-source-path src/ $(find src -name "*.java")
