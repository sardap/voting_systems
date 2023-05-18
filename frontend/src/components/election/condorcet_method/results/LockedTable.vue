<script setup lang="ts">
import type { CondorcetMethodResult } from '@/backend';
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
</script>

<template>
    <div>
        <table>
            <tr>
                <th>Name</th>
                <th>Locked in</th>
                <th></th>
            </tr>
            <tr v-for="(i, from) in result.locked_in_pairwise_victories" :class="i.length == 0 ? `unlocked` : ``">
                <td>{{ get_name(from) }}</td>
                <td>{{ i.length > 0 ? `Yes` : `No` }}</td>
                <td>{{ i.length > 0 ? `🔒` : `🔓` }}</td>
            </tr>
        </table>
    </div>
</template>

<style scoped>
.unlocked {
    background-color: #e74c3c;
}
</style>
