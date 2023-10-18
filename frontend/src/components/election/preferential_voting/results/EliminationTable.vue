<script setup lang="ts">
import type { LogEntry, PrefElectionResult } from '@/backend';
import { onMounted, ref, type PropType } from 'vue';

const props = defineProps({
    election_result: {
        required: true,
        type: Object as PropType<PrefElectionResult>,
    }
});

interface Candidate {
    name: String;
    original_index: number;
    eliminated_round: number | null;
}

let candidates_ordered = ref<Candidate[]>([]);

onMounted(async () => {
    const result = props.election_result;

    candidates_ordered.value = result.candidates.map((candidate_name, index) => {
        let eliminated_round: number | null = result.log.findIndex((round, i) => round.eliminated.includes(index));
        if (eliminated_round == -1) {
            eliminated_round = null;
        } else {
            eliminated_round++;
        }

        return {
            name: candidate_name,
            original_index: index,
            eliminated_round: eliminated_round,
        };
    });

    candidates_ordered.value.sort((a, b) => {
        const last_log = result.log[result.log.length - 1];
        const a_votes = last_log.votes[a.original_index].votes;
        const b_votes = last_log.votes[b.original_index].votes;

        if (b_votes == a_votes) {
            if (a.original_index == props.election_result.winner) {
                return -1;
            }

            if (b.original_index == props.election_result.winner) {
                return 1;
            }

            if (a.eliminated_round == null) {
                return -1;
            }
            if (b.eliminated_round == null) {
                return 1;
            }

            return b.eliminated_round - a.eliminated_round;
        }

        return b_votes - a_votes;
    });
});

function get_vote_percent(vote_count: number) {
    return ((vote_count / props.election_result.vote_count) * 100).toFixed(2);
}


function get_td_round_value(candidate: Candidate, round_number: number, round: LogEntry) {
    if (candidate.eliminated_round != null && round_number >= candidate.eliminated_round) {
        return "💀";
    }

    return get_vote_percent(round.votes[candidate.original_index].votes) + "%" + " (" + round.votes[candidate.original_index].votes + ")";
}

function get_td_winner_value(rank: number) {
    switch (rank) {
        case 0:
            return "🥇";
        case 1:
            return "🥈";
        case 2:
            return "🥉";
        default:
            return rank + 1;
    }
}

</script>

<template>
    <table>
        <tr>
            <th>Name</th>
            <th v-for="round in election_result.log.length">Round {{ round }}</th>
            <th>Rank</th>
        </tr>
        <tr v-for="[i, candidate] in candidates_ordered.entries()">
            <td>{{ candidate.name }}</td>
            <td v-for="[round_number, round] in election_result.log.entries()">
                {{ get_td_round_value(candidate, round_number + 1, round) }}</td>
            <td>{{ get_td_winner_value(i) }} </td>
        </tr>
    </table>
</template>

<style scoped></style>
