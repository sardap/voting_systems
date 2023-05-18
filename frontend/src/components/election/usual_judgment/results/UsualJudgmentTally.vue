<script setup lang="ts">
import { MJRating, type MajorityJudgmentTally, from_mj_rating_to_string, UJGrade, ujgrade_to_string, ElectionType } from '@/backend';
import { ref, type PropType } from 'vue';
import { Bar } from 'vue-chartjs';
import { Chart as ChartJS, Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale } from 'chart.js'

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale)


const props = defineProps({
    options: {
        type: Array as PropType<{ option_index: number, name: string }[]>,
        required: true,
    },
    tally: {
        type: Object as PropType<{ option_index: number, ratings: number[] }[]>,
        required: true,
    },
    vote_count: {
        type: Number,
        required: true,
    }
});

const ratings = [UJGrade.Bad, UJGrade.Inadequate, UJGrade.Fair, UJGrade.Good, UJGrade.VeryGood, UJGrade.Excellent];
const filtered_tally = ref(props.tally.filter((x) => props.options.map((i) => i.option_index).includes(x.option_index)));

function get_class(rating: UJGrade) {
    switch (rating) {
        case UJGrade.Bad:
            return `bad`;
        case UJGrade.Inadequate:
            return `inadequate`;
        case UJGrade.Fair:
            return `fair`;
        case UJGrade.Good:
            return `good`;
        case UJGrade.VeryGood:
            return `very-good`;
        case UJGrade.Excellent:
            return `excellent`;
    }
}

function get_rating_color(rating: UJGrade) {
    switch (rating) {
        case UJGrade.Bad:
            return '#8b0000';
        case UJGrade.Inadequate:
            return '#ff4500';
        case UJGrade.Passable:
            return '#ffa500';
        case UJGrade.Fair:
            return '#ffd700';
        case UJGrade.Good:
            return '#9acd32';
        case UJGrade.VeryGood:
            return '#008000';
        case UJGrade.Excellent:
            return '#006400';
    }
}

function make_dataset(rating: UJGrade) {
    const dataset = [];
    for (let i = 0; i < props.options.length; i++) {
        const option = props.options[i];
        const ratings = props.tally.find((x) => x.option_index === option.option_index)?.ratings as number[];
        dataset.push(ratings[rating]);
    }
    return { label: ujgrade_to_string(rating), data: dataset, backgroundColor: get_rating_color(rating) };
}

const bar_data = ref({
    labels: props.options.map((i) => i.name),
    datasets: Object.values(UJGrade).map((i) => make_dataset(i as UJGrade)),
});
const chart_options = ref({
    responsive: true,
    scales: {
        x: {
            stacked: true,
        },
        y: {
            stacked: true,
        }
    }
});

function find_median_category(category_sums: number[]): UJGrade {
    const expanded_list = [];
    for (const [index, value] of category_sums.entries()) {
        for (let i = 0; i < value; i++) {
            expanded_list.push(index);
        }
    }
    expanded_list.sort((a, b) => a - b);

    return expanded_list[Math.floor(expanded_list.length / 2)];
}

</script>

<template>
    <div>
        <h3>Table</h3>
        <table>
            <tr>
                <th>Name</th>
                <th v-for="rating in ratings">{{ ujgrade_to_string(rating) }}</th>
                <th>Median</th>
            </tr>
            <tr v-for="entry in filtered_tally">
                <td>{{ options.find((i) => i.option_index == entry.option_index)?.name }}</td>
                <td v-for="rating in ratings" :class="get_class(rating)">{{ entry.ratings[rating] }}</td>
                <td :style="`color: ` + get_rating_color(find_median_category(entry.ratings)) + `;`">{{
                    ujgrade_to_string(find_median_category(entry.ratings)) }}
                </td>
            </tr>
        </table>
        <h3>Chart</h3>
        <div>
            <Bar :options="chart_options" :data="bar_data" />
        </div>
    </div>
</template>

<style scoped></style>
