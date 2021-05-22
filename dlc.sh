#!/bin/bash
set -e

DOCKER_BUILDKIT=1 docker build \
	--target builder \
	--tag jsalverda/jarvis-modbus-exporter:dlc-builder \
	--cache-from jsalverda/jarvis-modbus-exporter:dlc-builder \
	--build-arg BUILDKIT_INLINE_CACHE=1 .
DOCKER_BUILDKIT=1 docker push jsalverda/jarvis-modbus-exporter:dlc-builder

DOCKER_BUILDKIT=1 docker build \
	--tag jsalverda/jarvis-modbus-exporter:dlc \
	--cache-from jsalverda/jarvis-modbus-exporter:dlc-builder \
	--cache-from jsalverda/jarvis-modbus-exporter:dlc \
	--build-arg BUILDKIT_INLINE_CACHE=1 .
DOCKER_BUILDKIT=1 docker push jsalverda/jarvis-modbus-exporter:dlc