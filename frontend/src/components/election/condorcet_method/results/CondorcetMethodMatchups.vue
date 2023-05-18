<script setup lang="ts">
import type { CondorcetMethodResult } from '@/backend';
import type { PropType } from 'vue';

defineProps({
    result: {
        type: Object as PropType<CondorcetMethodResult>,
        required: true,
    },
});

</script>

<template>
    <div>
        <div v-if="result.condorcet_winner">
            <p>Condorcet Winner was found!: {{ result.options[result.condorcet_winner] }}</p>
        </div>
        <table>
            <tr>
                <th>Name</th>
                <th v-for="[i, option] in result.options.entries()">{{ option }}{{ result.condorcet_winner == i ? "🥇" : ""
                }}</th>
            </tr>
            <tr v-for="[i, option] in result.options.entries()">
                <td>{{ option }}{{ result.condorcet_winner == i ? "🥇" : "" }}</td>
                <td v-for="[j, count] in result.matchups[i].entries()">{{ i == j ? "-" : count }}</td>
            </tr>
        </table>
    </div>
</template>

<style scoped>
.runoff {
    background-color: #74ef97;
}
</style>
