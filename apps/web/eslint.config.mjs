// @ts-check
import { config as baseConfig } from '@ponzu/eslint-config/base';
import withNuxt from './.nuxt/eslint.config.mjs'

export default withNuxt(
  baseConfig,
)
