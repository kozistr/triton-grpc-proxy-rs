#!/bin/bash
curl -H "Content-type:application/json" -X POST http://127.0.0.1:8080/v1/embedding -d "[{\"query\": \"asdf\"}, {\"query\": \"asdf asdf\"}, {\"query\": \"asdf asdf asdf\"}, {\"query\": \"asdf asdf asdf asdf\"}]"
