import { arg, defineCommand } from 'yuzuha';

export default defineCommand({
  name: 'eco',
  description: 'Repete o texto informado.',
  args: { texto: arg.text().rest().required() },
  async run(ctx) {
    await ctx.reply(ctx.args.get('texto'));
  },
});
