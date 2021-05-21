#!/bin/bash
set -e

docker build \
	--target planner \
	--tag jsalverda/jarvis-modbus-exporter:dlc-main-planner \
	--cache-from jsalverda/jarvis-modbus-exporter:dlc-main-planner \
	--build-arg BUILDKIT_INLINE_CACHE=1 .
docker push jsalverda/jarvis-modbus-exporter:dlc-main-planner

docker build \
	--target cacher \
	--tag jsalverda/jarvis-modbus-exporter:dlc-main-cacher \
	--cache-from jsalverda/jarvis-modbus-exporter:dlc-main-planner \
	--cache-from jsalverda/jarvis-modbus-exporter:dlc-main-cacher \
	--build-arg BUILDKIT_INLINE_CACHE=1 .
docker push jsalverda/jarvis-modbus-exporter:dlc-main-cacher

docker build \
	--target builder \
	--tag jsalverda/jarvis-modbus-exporter:dlc-main-builder \
	--cache-from jsalverda/jarvis-modbus-exporter:dlc-main-planner \
	--cache-from jsalverda/jarvis-modbus-exporter:dlc-main-cacher \
	--cache-from jsalverda/jarvis-modbus-exporter:dlc-main-builder \
	--build-arg BUILDKIT_INLINE_CACHE=1 .
docker push jsalverda/jarvis-modbus-exporter:dlc-main-builder

docker build \
	--target runtime \
	--tag jsalverda/jarvis-modbus-exporter:dlc-main-runtime \
	--cache-from jsalverda/jarvis-modbus-exporter:dlc-main-planner \
	--cache-from jsalverda/jarvis-modbus-exporter:dlc-main-cacher \
	--cache-from jsalverda/jarvis-modbus-exporter:dlc-main-builder \
	--cache-from jsalverda/jarvis-modbus-exporter:dlc-main-runtime \
	--build-arg BUILDKIT_INLINE_CACHE=1 .
docker push jsalverda/jarvis-modbus-exporter:dlc-main-runtime
