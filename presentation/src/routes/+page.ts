import { invoke } from "@tauri-apps/api/core";

type QuestionerStats = {
    high_score: number,
    daily_streak: number,
    previous_score: number
}

function getStats(): Promise<QuestionerStats> {
    return invoke("get_stats")
}


export const load = async () => {
    const stats = await getStats();

    return {
        stats
    };
};