<script setup lang="ts">
import type { PrefElectionResult, VoteLogEntry } from '@/backend';
import { onMounted, ref, type PropType } from 'vue';
import { Bar } from 'vue-chartjs';
import { Chart as ChartJS, Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale, plugins } from 'chart.js'

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale)

const props = defineProps({
    election_result: {
        required: true,
        type: Object as PropType<PrefElectionResult>,
    }
});

function get_preference_flow(vote: VoteLogEntry) {
    for (let i = props.election_result.log.length - 1; i >= 0; i--) {
        const log = props.election_result.log[i];
        for (let j = 0; j < vote.votes.length; j++) {
            if (!log.eliminated.includes(vote.votes[j])) {
                return vote.votes[j];
            }
        }
    }

    return vote.votes[0];
}

interface FlowEntry {
    candidate: string,
    percentage: number,
}

interface Flow {
    first_preference: string;
    entires: FlowEntry[];
}

let flows = ref<Flow[]>([]);

onMounted(async () => {

    const result = new Map<string, number[]>();

    // props.election_result.votes.forEach((vote) => {
    //     const first_preference = vote.votes[0];
    //     const flowed_preference = get_preference_flow(vote);
    //     if (first_preference === flowed_preference) {
    //         return;
    //     }
    //     let first_pref_candidates = props.election_result.candidates[first_preference];

    //     let first_preference_flow = result.get(first_pref_candidates);
    //     if (!first_preference_flow) {
    //         first_preference_flow = props.election_result.candidates.map(_ => 0);
    //     }

    //     first_preference_flow[flowed_preference]++;
    //     result.set(first_pref_candidates, first_preference_flow);
    // });

    flows.value = props.election_result.candidates.filter((candidate_name) => {
        return result.has(candidate_name);
    }).map((candidate_name) => {
        const candidate_counts = result.get(candidate_name) as number[];

        const candidate_count_sum = candidate_counts.reduce((a, b) => a + b, 0);

        const flow: Flow = {
            first_preference: candidate_name,
            entires: candidate_counts.map((count, index) => {
                return {
                    candidate: props.election_result.candidates[index],
                    percentage: count / candidate_count_sum,
                };
            }).filter((entry) => entry.percentage > 0),
        };

        return flow;
    });
});

function background_colours(candidate: string, candidates: string[]) {
    const index = props.election_result.candidates.indexOf(candidate);
    const other_name = candidates.find((c) => c !== candidate) as string;
    const other_index = props.election_result.candidates.indexOf(other_name);

    const colors: string[] = ["#3F51B5", "#F44336"];

    if (index > other_index) {
        return colors[0];
    }

    return colors[1];
}

function create_bar_data(flow: Flow) {
    const candidates = flow.entires.map((entry) => entry.candidate)
    return {
        labels: ['flowed to'],
        datasets: flow.entires.map((entry) => {
            return { data: [entry.percentage * 100], backgroundColor: background_colours(entry.candidate, candidates), label: entry.candidate }
        })
    }

}

</script>

<template>
    <div v-if="flows.length == 0">
        <p>No preference flows</p>
    </div>
    <div v-else>
        <div v-for="flow in flows">
            <h3>{{ flow.first_preference }}</h3>
            <Bar :options="{ indexAxis: 'y', scales: { y: { stacked: true }, x: { stacked: true } }, plugins: { legend: { display: true } } }"
                :data="create_bar_data(flow)" />
        </div>
    </div>
</template>

<style scoped></style>
