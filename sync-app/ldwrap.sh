#!/bin/bash

for arg do
  shift
  case $arg in
    (-lgcc_s) : ;;
       (*) set -- "$@" "$arg" ;;
  esac
done

exec arm-unknown-linux-gnueabi-gcc "$@"
