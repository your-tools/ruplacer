set -x
set -e

main() {
   if [[ "${TRAVIS_OS_NAME}" == "windows" ]]; then
      choco install python
      /C/Python37/python.exe ci/gen_artifacts.py --version $TRAVIS_TAG
   else
      python3 ci/gen_artifacts.py --version $TRAVIS_TAG
   fi
}

main()
