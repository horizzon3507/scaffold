import { defineCommand } from 'yuzuha';

export default defineCommand({
  name: 'ping',
  description: 'Responde com pong.',
  async run(ctx) {
    await ctx.reply('pong');
  },
});
