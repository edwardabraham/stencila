import { fromJSON, toJSON } from './prelude'

const addon = require('../native')

type LoggingLevel = 'debug' | 'info' | 'warn' | 'error' | 'never'

export interface Config {
  logging: {
    stderr: {
      level: LoggingLevel
      format: 'plain' | 'pretty' | 'json'
    }
    file: {
      path: string
      level: LoggingLevel
    }
  }
  serve: {
    url: string
    key?: string
    insecure: boolean
  }
  plugins: {
    kinds: Array<'docker' | 'binary' | 'package'>
    aliases: Record<string, string>
  }
  upgrade: {
    confirm: boolean
    verbose: boolean
    auto: string
  }
}

/**
 * Read the configuration from the configuration file
 *
 * @returns The configuration object
 */
export function read(): Config {
  return fromJSON<Config>(addon.configRead())
}

/**
 * Write the configuration to the configuration file
 *
 * @param config The configuration object
 */
export function write(config: Config): void {
  addon.configWrite(toJSON(config))
}

/**
 * Test that the configuration object is valid
 *
 * @param config
 * @returns true (or throws an error)
 */
export function validate(config: Config): true {
  return addon.configValidate(toJSON(config))
}

/**
 * Set a property of the configuration object
 *
 * Performs validation on the value. Will throw errors for invalid pointer
 * or error.
 *
 * @param config The configuration object
 * @param pointer The pointer to the property to be set e.g. `upgrade.auto`
 * @param value The value to set the property to
 * @returns The updated configuration object
 */
export function set(config: Config, pointer: string, value: string): Config {
  return fromJSON<Config>(addon.configSet(toJSON(config), pointer, value))
}

/**
 * Reset all or part of the configuration to defaults
 *
 * @param config The configuration object
 * @param property The property to reset. Use `all` to reset the entire object.
 * @returns The updated configuration object
 */
export function reset(
  config: Config,
  property: 'all' | 'logging' | 'serve' | 'plugins' | 'upgrade'
): Config {
  return fromJSON<Config>(addon.configReset(toJSON(config), property))
}