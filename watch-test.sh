#!/bin/bash
watchexec --exts rs,toml --clear --restart 'cargo test -- --nocapture'