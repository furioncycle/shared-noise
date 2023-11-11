#!/bin/bash

fuser -k 3000/tcp
fuser -k 3001/tcp

cargo leptos watch
