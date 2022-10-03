import { Client, Guild } from "discord.js";

export async function get_user_name(client: Client, guild: Guild | null, uid: string): Promise<string> {
    /**
     * Attempts to retrieve the Discord display username of a given user id.
     * 
     * @remarks If the given user is in the given guild, we try to get their nickname. If not, simply 
     * return their global Discord username. Even if a user is in a guild, they sometimes might not 
     * show up in the guild members cache (for instance, if they are offline).
     * 
     * @param client - The Discord Client instance used to query the information.
     * @param guild - The Discord Guild (or server) which the user might belong to.
     * @param uid - The user id of the target user.
     * 
     * @returns A Promise of either the username, or a static string in the event of an error.
     */
    try {
        let name = (await client.users.fetch(uid)).username;
        if (guild) {
            let guild_user = guild.members.cache.get(uid);
            if (guild_user !== undefined && guild_user.nickname) {
                name = guild_user.nickname;
            }
        }
        return name;
    }
    catch (e: any) {
        return "*???*";
    }
}

export async function get_user_avatar(client: Client, guild: Guild | null, uid: string): Promise<string> {
    /**
     * Fetches an appropriate user avatar for the given uid, 
     * prioritizing a guild icon, if it exists and is cached.
     * 
     * @param client - The Discord Client instance used to query the information.
     * @param guild - The Discord Guild (or server) which the user might belong to.
     * @param uid - The user id of the target user.
     * 
     * @returns A Promise of either the URL of the avatar image, or a static string in the event of an error.
     */
    try {
        let avatar = (await client.users.fetch(uid)).displayAvatarURL();
        if (guild) {
            let guild_user = guild.members.cache.get(uid);
            if (guild_user !== undefined && guild_user.displayAvatarURL()) {
                avatar = guild_user.displayAvatarURL();
            }
        }
        return avatar;
    }
    catch (e: any) {
        return "*???*";
    }
}