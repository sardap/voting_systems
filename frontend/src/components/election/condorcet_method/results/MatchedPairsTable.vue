<script setup lang="ts">
import type { CondorcetMethodResult, MatchedPair } from '@/backend';
import type { PropType } from 'vue';

const props = defineProps({
    result: {
        type: Object as PropType<CondorcetMethodResult>,
        required: true,
    },
});

function get_name(index: number) {
    if (props.result.matched_pair_winner == index) {
        return props.result.options[index] + " 🥇";
    } else {
        return props.result.options[index];
    }
}

function get_pair_winner(pair: MatchedPair) {
    if (pair.difference > 0) {
        return get_name(pair.runner);
    } else if (pair.difference < 0) {
        return get_name(pair.opponent);
    } else {
        return "Tie";
    }
}

</script>

<template>
    <div v-if="result.matched_pairs">
        <table>
            <tr>
                <th>Runner</th>
                <th>Opponent</th>
                <th>Difference</th>
                <th>Winner</th>
            </tr>
            <tr v-for="i in result.matched_pairs">
                <td>{{ get_name(i.runner) }}: {{ i.votes_for_runner }}</td>
                <td>{{ get_name(i.opponent) }}: {{ i.votes_for_opponent }}</td>
                <td>{{ i.difference }}</td>
                <td>{{ get_pair_winner(i) }}</td>
            </tr>
        </table>
    </div>
</template>

<style scoped>
.runoff {
    background-color: #74ef97;
}
</style>
