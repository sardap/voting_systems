<script setup lang="ts">
import { MJRating, from_mj_rating_to_string } from '@/backend'
import { ref, type PropType } from 'vue'
import { Bar } from 'vue-chartjs'
import {
  Chart as ChartJS,
  Title,
  Tooltip,
  Legend,
  BarElement,
  CategoryScale,
  LinearScale
} from 'chart.js'

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale)

const props = defineProps({
  options: {
    type: Array as PropType<{ option_index: number; name: string }[]>,
    required: true
  },
  tally: {
    type: Object as PropType<{ option_index: number; ratings: number[] }[]>,
    required: true
  },
  vote_count: {
    type: Number,
    required: true
  }
})

const ratings = [
  MJRating.VeryGood,
  MJRating.Good,
  MJRating.Acceptable,
  MJRating.Poor,
  MJRating.Terrible
]
const filtered_tally = ref(
  props.tally.filter((x) => props.options.map((i) => i.option_index).includes(x.option_index))
)

function get_class(rating: MJRating) {
  switch (rating) {
    case MJRating.VeryGood:
      return `very-good`
    case MJRating.Good:
      return `good`
    case MJRating.Acceptable:
      return `acceptable`
    case MJRating.Poor:
      return `poor`
    case MJRating.Terrible:
      return `terrible`
  }
}

function get_rating_color(rating: MJRating) {
  switch (rating) {
    case MJRating.VeryGood:
      return 'green'
    case MJRating.Good:
      return '#4CAF50'
    case MJRating.Acceptable:
      return '#FF9800'
    case MJRating.Poor:
      return '#F44336'
    case MJRating.Terrible:
      return '#ff0000'
  }
}

function make_dataset(rating: MJRating) {
  const dataset = []
  for (let i = 0; i < props.options.length; i++) {
    const option = props.options[i]
    const ratings = props.tally.find((x) => x.option_index === option.option_index)
      ?.ratings as number[]
    dataset.push(ratings[rating])
  }
  return {
    label: from_mj_rating_to_string(rating),
    data: dataset,
    backgroundColor: get_rating_color(rating)
  }
}

const bar_data = ref({
  labels: props.options.map((i) => i.name),
  datasets: [
    make_dataset(MJRating.VeryGood),
    make_dataset(MJRating.Good),
    make_dataset(MJRating.Acceptable),
    make_dataset(MJRating.Poor),
    make_dataset(MJRating.Terrible)
  ]
})
const chart_options = ref({
  responsive: true,
  scales: {
    x: {
      stacked: true
    },
    y: {
      stacked: true,
      ticks: {
        callback: function (value: string, index: number, ticks: number) {
          if (Number(value) == props.vote_count / 2) {
            return `MEDIAN POINT: ${value}`
          }
          return value
        }
      }
    }
  }
})

function find_median_category(category_sums: number[]): MJRating {
  const expanded_list = []
  for (const [index, value] of category_sums.entries()) {
    for (let i = 0; i < value; i++) {
      expanded_list.push(index)
    }
  }
  expanded_list.sort((a, b) => a - b)

  return expanded_list[Math.floor(expanded_list.length / 2)]
}
</script>

<template>
  <div>
    <h3>Table</h3>
    <table>
      <tr>
        <th>Name</th>
        <th v-for="(rating, i) in ratings" :key="i">{{ from_mj_rating_to_string(rating) }}</th>
        <th>Median</th>
      </tr>
      <tr v-for="entry in filtered_tally" :key="entry.option_index">
        <td>{{ options.find((i) => i.option_index == entry.option_index)?.name }}</td>
        <td v-for="(rating, j) in ratings" :class="get_class(rating)" :key="j">
          {{ entry.ratings[rating] }}
        </td>
        <td :style="`color: ` + get_rating_color(find_median_category(entry.ratings)) + `;`">
          {{ from_mj_rating_to_string(find_median_category(entry.ratings)) }}
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
