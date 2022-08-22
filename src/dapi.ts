import { Client, Guild } from "discord.js";

export async function get_username(client: Client, guild: Guild | null, uid: string): Promise<string> {
    let username = (await client.users.fetch(uid)).username;
    if (guild) {
        let guild_user = guild.members.cache.get(uid);
        if (guild_user !== undefined && guild_user.nickname) {
            username = guild_user.nickname;
        }
    }
    return username;
}