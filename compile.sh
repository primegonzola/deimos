#!/bin/bash
glslc -fshader-stage=vert ./shaders/blit_vs.glsl -o ./shaders/compiled/blit_vs.spv
glslc -fshader-stage=frag ./shaders/blit_fs.glsl -o ./shaders/compiled/blit_fs.spv

glslc -fshader-stage=vert ./shaders/sample_vs.glsl -o ./shaders/compiled/sample_vs.spv
glslc -fshader-stage=frag ./shaders/sample_fs.glsl -o ./shaders/compiled/sample_fs.spv
