<script setup lang="ts">
import type { ThreeTwoOneResult } from '@/backend';
import type { PropType } from 'vue';

defineProps({
    result: {
        type: Object as PropType<ThreeTwoOneResult>,
        required: true,
    },
});

function vote_to_string(vote: number): string {
    if (vote == 0) {
        return "😞 Bad";
    } else if (vote == 1) {
        return "🤔 Ok";
    }
    return "😀 Good";
}
</script>

<template>
    <div>
        <table>
            <tr>
                <th>Count</th>
                <th v-for="name in result.options">{{ name }}</th>
            </tr>
            <tr v-for="tally in result.vote_tally">
                <td>{{ tally.count }}</td>
                <td v-for="vote in tally.votes">{{ vote_to_string(vote) }}</td>
            </tr>
        </table>
    </div>
</template>

<style scoped></style>
