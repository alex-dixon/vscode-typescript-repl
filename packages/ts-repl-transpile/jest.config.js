module.exports = {
  verbose: true,
  testEnvironment: 'node',
  preset: 'ts-jest',
  testMatch: ['<rootDir>/test/**/*.ts'],
  testPathIgnorePatterns: ['test/lib'],
  passWithNoTests: true,
}

