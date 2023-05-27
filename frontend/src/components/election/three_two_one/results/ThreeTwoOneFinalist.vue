<script setup lang="ts">
import type { ThreeTwoOneResult } from '@/backend';
import type { PropType } from 'vue';

const props = defineProps({
    result: {
        type: Object as PropType<ThreeTwoOneResult>,
        required: true,
    },
});

function get_vote_count(option_index: number, other_option_index: number): number {
    let count = 0;
    for (const vote of props.result.vote_tally) {
        const a = vote.votes[option_index];
        const b = vote.votes[other_option_index];
        if (a > b) {
            count += vote.count;
        }
    }

    return count;
}

const a_count = get_vote_count(props.result.finalists[1], props.result.finalists[0]);
const b_count = get_vote_count(props.result.finalists[0], props.result.finalists[1]);
const total_count = a_count + b_count;


</script>

<template>
    <div>
        <table>
            <tr>
                <th>Name</th>
                <th>In Favour</th>
                <th>Winner</th>
            </tr>
            <tr>
                <td>{{ result.options[result.finalists[0]] }}</td>
                <td>{{ (a_count / total_count * 100).toFixed() }}% ({{ a_count }})</td>
                <td>{{ result.winner == result.finalists[0] ? "✅" : "❌" }}</td>
            </tr>
            <tr>
                <td>{{ result.options[result.finalists[1]] }}</td>
                <td>{{ (b_count / total_count * 100).toFixed() }}% ({{ b_count }})</td>
                <td>{{ result.winner == result.finalists[1] ? "✅" : "❌" }}</td>
            </tr>
        </table>
    </div>
</template>

<style scoped></style>
