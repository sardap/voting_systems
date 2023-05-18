<script setup lang="ts">
import { ref, type PropType } from 'vue';

interface VoteOption {
    name: string;
    index: number;
    approve: boolean;
}

const props = defineProps({
    modelValue: {
        type: Object as PropType<VoteOption[]>,
        required: true,
    },
    check_box_title: {
        type: String,
        required: false,
        default: "Approve",
    },
    single_option: {
        type: Boolean,
        required: false,
        default: false,
    }
});

const emits = defineEmits<{
    (e: 'update:modelValue', value: VoteOption[]): void
}>();


const options = ref<VoteOption[]>(props.modelValue);

function refresh_options(index: number) {
    if (props.single_option) {
        for (let i = 0; i < options.value.length; i++) {
            if (i != index) {
                options.value[i].approve = false;
            }
        }
    }

    emits(`update:modelValue`, options.value);
}

function toggle(index: number) {
    options.value[index].approve = !options.value[index].approve;
    refresh_options(index);
}

</script>

<template>
    <div class="table">
        <div class="row header">
            <div class="col rank">
                <p>{{ check_box_title }}</p>
            </div>
            <div class="col">
                <p>Name</p>
            </div>
        </div>
        <div v-for="(option, i) in options" :key="option.index" class="row item" @click="toggle(i)">
            <div class="col rank">
                <input type="checkbox" v-model="options[i].approve" @change="refresh_options(i)" />
            </div>
            <div class="col">
                <p>{{ option.name }}</p>
            </div>
        </div>
    </div>
</template>

<style scoped>
div.row:hover {
    background-color: #E8AB8C;
}

div.row:nth-child(2n):hover {
    background-color: #E8AB8C;
}

div.row:nth-child(2n) {
    background-color: #FAD8C0;
}

div.row {
    background-color: #FDE5D0;
}

div.row.header {
    background-color: #A2F4B9;
}

div.table {
    display: table;
    max-width: 800px;
    width: 70%;
    border-collapse: separate;
    border-spacing: 0;
}

div.row {
    display: table-row;
}

div.col {
    display: table-cell;
    border: 1px solid black;
    padding: 8px;
    text-align: left;
    vertical-align: top;
}

div.col.rank {
    width: 10px;
}

.submit {
    margin-top: 20px;
    max-width: 800px;
    width: 70%;
    height: 80px;
}
</style>
