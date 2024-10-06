// @ts-check
import antfu from '@antfu/eslint-config'

export default antfu({
  ignores: ['wasm/pkg/*'],
  typescript: {
    tsconfigPath: './tsconfig.json',
  },
  rules: {
    'antfu/top-level-function': 'off',
  },
})
