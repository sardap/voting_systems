<script setup lang="ts">
import { onMounted, ref, type PropType } from 'vue';

interface ElectionVote {
    votes: number[];
}

interface PreferenceTally {
    candidates: string[];
    votes: ElectionVote[];
}

const props = defineProps({
    election_result: {
        required: true,
        type: Object as PropType<PreferenceTally>,
    }
});

onMounted(async () => {
});

function get_n_pref_count(candidate_index: number, pref_number: number) {
    const votes = props.election_result.votes;

    let count = 0;

    for (let i = 0; i < votes.length; i++) {
        if (candidate_index == votes[i].votes[pref_number - 1]) {
            count++;
        }
    }

    return count;
}


</script>

<template>
    <table>
        <tr>
            <th>Name</th>
            <th v-for="i in election_result.candidates.length">{{ i }}</th>
        </tr>
        <tr v-for="[can_idx, candidate] in election_result.candidates.entries()">
            <td class="first-col">{{ candidate }}</td>
            <td v-for="pref_number in election_result.candidates.length">{{ (get_n_pref_count(can_idx, pref_number) /
                election_result.votes.length * 100).toFixed(2) }}% ({{ get_n_pref_count(can_idx, pref_number) }})</td>
        </tr>
    </table>
</template>

<style scoped></style>
