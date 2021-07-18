module.exports = {
  testEnvironment: 'jsdom',
  setupFilesAfterEnv: ['<rootDir>/setup-test-env.js'],
  transform: {
    '^.+\\.jsx?$': '<rootDir>/jest-preprocess.js'
  },
  moduleNameMapper: {
    '^.+\\.(css|styl|less|sass|scss)$': 'identity-obj-proxy',
    '.+\\.(jpg|jpeg|png|gif|eot|otf|webp|svg|ttf|woff|woff2|mp4|webm|wav|mp3|m4a|aac|oga)$': '<rootDir>/__mocks/file-mock.js',
  },
  testPathIgnorePatterns: ['node_modules', '\\.cache', '<rootDir>.*/public'],
  transformIgnorePatterns: ['node_modules/(?!(gatsby|d3|d3-.+|internmap|delaunator|robust-predicates)/)'],
  globals: {
    __PATH_PREFIX__: '',
  },
  setupFiles: ['<rootDir>loadershim.js']
};