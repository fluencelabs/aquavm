#
#  Copyright 2023 Fluence Labs Limited
#
#  Licensed under the Apache License, Version 2.0 (the "License");
#  you may not use this file except in compliance with the License.
#  You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
#  Unless required by applicable law or agreed to in writing, software
#  distributed under the License is distributed on an "AS IS" BASIS,
#  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#  See the License for the specific language governing permissions and
#  limitations under the License.
#
from setuptools import setup


setup(name='aquavm_performance_metering',
      version='0.1',
      description='An AquaVM Performance metering tool',
      author='Fluence Labs Limited',
      license='Apache-2.0',
      packages=['performance_metering'],
      zip_safe=True,
      entry_points={
          'console_scripts': [
              'aquavm_performance_metering=performance_metering.main:main',
          ],
      })
