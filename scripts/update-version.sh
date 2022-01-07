#!/bin/bash

set -o errexit
set -o errtrace

# How to install `fd`: https://github.com/sharkdp/fd#installation
: "${FD:=fd}"

if command -v gsed > /dev/null 2>&1
then
  : "${SED:=gsed}"
fi

: "${SED:=sed}"

CURRENT_VERSION="1.1.0"

if [[ -z "$1" ]]
then
  echo "Usage: $0 <new-version>"
  echo
  echo "# Arguments"
  echo "  new-version  A semver compliant version number"

  exit 1
fi

if [[ "$1" == "--get" || "$1" == "-g" ]]
then
  echo ${CURRENT_VERSION}

  exit 0
fi

NEW_VERSION="$1"

echo "Current version: $CURRENT_VERSION"
echo "New version: $NEW_VERSION"
echo "Using \`fd\`: $FD"
echo "Using \`sed\`: $SED"

${FD} Cargo.toml --exec ${SED} -i '{}' -e "s/version = \"${CURRENT_VERSION}\"$/version = \"${NEW_VERSION}\"/"
echo "manually check changes to \`Cargo.toml\`"

${FD} setup.py --exec ${SED} -i '{}' -e "s/version='${CURRENT_VERSION}',\?$/version='${NEW_VERSION}',/"
echo "manually check changes to \`setup.py\`"

${FD} update-version.sh --exec ${SED} -i '{}' -e "s/CURRENT_VERSION='${CURRENT_VERSION}',\?$/CURRENT_VERSION='${NEW_VERSION}',/"
echo "manually check changes to \`update-version.sh\`"
