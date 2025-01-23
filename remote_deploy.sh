#!/bin/bash

docker exec nginx nginx -t
docker exec nginx nginx -s reload

