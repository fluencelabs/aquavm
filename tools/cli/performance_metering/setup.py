from setuptools import setup


setup(name='aquavm_performance_metering',
      version='0.1',
      description='An AquaVM Performance metering tool',
      author='Fluence Labs',
      author_email='devs@fluencelabs.one',
      license='Apache',
      packages=['performance_metering'],
      zip_safe=True,
      entry_points={
          'console_scripts': [
              'aquavm_performance_metering=performance_metering.main:main',
          ],
      })
