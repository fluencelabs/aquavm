#
# AquaVM Workflow Engine
#
# Copyright (C) 2024 Fluence DAO
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as
# published by the Free Software Foundation version 3 of the
# License.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.
#
from setuptools import setup


setup(name='aquavm_performance_metering',
      version='0.1',
      description='An AquaVM Performance metering tool',
      author='Fluence DAO',
      license='AGPL-3.0-only',
      packages=['performance_metering'],
      zip_safe=True,
      install_requires=[
          'humanize',
          # python 3.11 use standard tomllib, but it is not yet available
          # everywhere.
          'toml',
      ],
      entry_points={
          'console_scripts': [
              'aquavm_performance_metering=performance_metering.main:main',
          ],
      })
