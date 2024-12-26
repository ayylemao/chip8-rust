#!/bin/bash
# Move contents of pkg to docs and commit
rm -rf docs
mkdir docs
cp -r pkg/* docs/
git add docs/
git commit -m "Deploy to GitHub Pages"
git push

