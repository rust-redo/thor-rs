import test from 'ava'

import { getNpmAgent } from '../index.js'

test('get npm agent', (t) => {
  const match = process.env.npm_config_user_agent.match(/^(yarn|pnpm|npm)\/(\S+)/)
  t.deepEqual(getNpmAgent(process.env.npm_config_user_agent), {agent: match[1], version: match[2]})
})
