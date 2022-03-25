#!/bin/sh

set -e

git_ensure_clean() {
  if [ ! -z "$(git status --porcelain --untracked-files=no)" ]; then
    echo "Git directory is not clean. Aborting."
    exit 1;
  fi
}

git_ensure_main() {
  branch=$(git branch --show-current)
  if [ "main" != "$branch" ]; then
    echo "Not in main branch."
    exit 1;
  fi
}

get_crate_version() {
  # Basic pattern matching to extract crate version. Will definitely break one day.
  version=$(sed -E -n 's/^version = "([[:digit:]]\.[[:digit:]]\.[[:digit:]])"\s*$/\1/p' < "$1")
  if [ -z "$version" ]; then
    echo "Failed to identify metrics-fn version."
    exit 1;
  else
    echo "$version"
  fi
}

get_version_major_minor() {
  echo "$1" | sed -E -n 's/^([[:digit:]]\.[[:digit:]])\..*/\1/p'
}

replace_dependency() {
  toml="$1"
  name="$2"
  new_value="$3"

  sed -E "s/^$name\s*=\s*\{.+\}/$name = $new_value/" < "$toml"
}

prompt() {
  while true; do
      read -p "$1 " yn
      case $yn in
          [Yy]* ) return 0; break;;
          [Nn]* ) return 1;;
          * ) echo "Please answer yes or no.";;
      esac
  done
}


git_ensure_clean
git_ensure_main

toml="Cargo.toml"
crate_version=$(get_crate_version "$toml")
majmin_version=$(get_version_major_minor "$crate_version")
dep_name='metrics-fn-codegen'
dep_development_value='{ path = "../metrics-fn-codegen" }'
dep_development="$dep_name = $dep_development_value"
dep_publish_value="{ version = \"$majmin_version\" }"
git_tag="v$crate_version"
git_commit_msg="Release $crate_version."
toml_original=$(cat "$toml")
toml_publish=$(replace_dependency "$toml" "$dep_name" "$dep_publish_value")

echo "$toml_publish"
echo
echo "========================================================================================"
echo "Validate the above information carefully before publishing version $crate_version."
if ! prompt 'Would you like to proceed?'; then
  echo "Aborted."
fi

echo "$toml_publish" > "$toml"
if ! cargo publish --dry-run --allow-dirty; then
  git restore "$toml"
  echo "No changes performed. Aborted."
  exit 1
fi

echo "Commiting and tagging."
git add "$toml"
git commit -m "$git_commit_msg"
git tag "$git_tag"
git push --all