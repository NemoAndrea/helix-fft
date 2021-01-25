<template>
    <div>
        <div class="helix-title">
            <div class="card-title">Helix parameters</div>
        </div>
        <v-form
                ref="form"
                v-model="valid">
            <div v-for="(helix, index) in helixFamily" :key="index">
                <div class="helix-header">
                    <div class="helix-number" @click="setInFocus(index + 1)">
                        <div>helix {{index+1}}</div>
                        <div class="helix-button-box">
                            <div v-if="index + 1 !==inputInFocus">
                                <v-icon> mdi-chevron-down </v-icon>
                            </div>
                            <div v-if="index + 1 ===inputInFocus">
                                <v-icon> mdi-chevron-up</v-icon>
                            </div>
                        </div>
                    </div>
                    <div v-if="index + 1 ===inputInFocus" class="helix-button-box-focus">
                        <div class="handedness-button">
                            <v-tooltip bottom v-if="helix['handedness'] === 'right'">
                                <template v-slot:activator="{ on }">
                                    <v-icon color="var(--bw-button-color)" v-on="on" @click="toggleHandedness(helix)"> mdi-axis-z-rotate-counterclockwise</v-icon>
                                </template>
                                <span>Make left-handed helix</span>
                            </v-tooltip>
                            <v-tooltip bottom v-if="helix['handedness'] === 'left'">
                                <template v-slot:activator="{ on }">
                                    <v-icon color="var(--bw-button-color)" v-on="on" @click="toggleHandedness(helix)"> mdi-axis-z-rotate-clockwise</v-icon>
                                </template>
                                <span>Make right handed helix</span>
                            </v-tooltip>
                        </div>
                        <div class="delete-button">
                            <v-tooltip bottom>
                                <template v-slot:activator="{ on }">
                                    <v-icon color="var(--bw-button-color)" v-on="on" @click="resetParameters(index)"> mdi-minus-circle </v-icon>
                                </template>
                                <span>Delete helix</span>
                            </v-tooltip>
                        </div>
                        <div class="copy-button">
                            <v-tooltip bottom>
                                <template v-slot:activator="{ on }">
                                    <v-icon color="var(--bw-button-color)" v-on="on" @click="copyHelixParams(index)"> mdi-content-copy</v-icon>
                                </template>
                                <span>Copy params to new helix</span>
                            </v-tooltip>
                        </div>

                    </div>
                </div>
                <transition name="fade">
                    <div v-show="index+1 === inputInFocus">
                        <v-text-field
                                v-model="helix['radius']"
                                :rules="numberRules"
                                label="Radius"
                                required
                                outlined
                                color="var(--primary)"
                                :suffix=unit.label
                                type="number"
                        ></v-text-field>

                        <v-text-field
                                v-model="helix['rise']"
                                :rules="numberRules"
                                label="Rise"
                                required
                                outlined
                                color="var(--primary)"
                                :suffix=unit.label
                                type="number"
                        ></v-text-field>

                        <v-text-field
                                v-model="helix['frequency']"
                                :rules="numberRules"
                                label="Subunits per pitch"
                                required
                                outlined
                                color="var(--primary)"
                                type="number"
                        ></v-text-field>

                        <v-text-field
                                v-model="helix['unit_size']"
                                :rules="numberRules"
                                label="Unit size"
                                required
                                outlined
                                color="var(--primary)"
                                :suffix=unit.label
                                type="number"
                        ></v-text-field>

                        <div v-show="index+1 > 1" class="optional-params">
                            <div class="mini-field">
                            <v-text-field

                                    v-model="helix['offset']"
                                    :rules="numberAndZeroRules"
                                    label="Helix Z offset"
                                    outlined
                                    required
                                    color="var(--primary)"
                                    :suffix=unit.label
                                    type="number"
                            ></v-text-field>
                            </div>
                            <div class="mini-field">
                            <v-text-field
                                    v-model="helix['rotation']"
                                    :rules="anyNumberRules"
                                    label="Rotation"
                                    outlined
                                    required
                                    color="var(--primary)"
                                    suffix="deg"
                                    type="number"
                            ></v-text-field>
                            </div>
                        </div>
                    </div>
                </transition>
            </div>
            <div class="button-box">
                <v-tooltip bottom>
                    <template v-slot:activator="{ on }">
                        <v-btn
                                dark
                                fab
                                color="var(--primary)"
                                @click="newHelix"
                                v-on="on"
                        >
                            <v-icon>mdi-plus</v-icon>
                        </v-btn>
                    </template>
                    <span>Add another helix</span>
                </v-tooltip>
                <div class="button-box">
                    <v-tooltip bottom>
                        <template v-slot:activator="{ on }">
                        <span v-on="on">
                        <v-checkbox v-model="liveCompute"
                                    label="live"
                                    color="var(--primary)"
                                    class="live-button"
                        />
                            </span>
                        </template>
                        <span>Automatically compute FFT on parameter change</span>
                    </v-tooltip>
                <v-tooltip bottom>
                    <template v-slot:activator="{ on }">
                        <v-btn
                                :disabled="!valid || liveCompute"
                                color="var(--primary)"
                                @click="computeHelix(true)"
                                v-on="on">
                            Compute
                        </v-btn>
                    </template>
                    <span>Show helix and compute FFT</span>
                </v-tooltip>

                </div>
            </div>
        </v-form>
        <v-snackbar
                v-model="snackbar"
        >
            {{ snackText }}
            <template v-slot:action="{ attrs }">
                <v-btn
                        color="pink"
                        text
                        v-bind="attrs"
                        @click="snackbar = false"
                >
                    Close
                </v-btn>
            </template>
        </v-snackbar>
    </div>
</template>

<script>
    export default {
        name: "ParameterPanel",
        props: {
            unit: {
                type: Object,
                default: function () { return {label:'Å', factor: 0.1} }
                },
        },
        data: () => ({
            valid: false,
            liveCompute: false,
            numberRules: [
                v => !!v || 'parameter is required',
                v => v > 0 || 'value must be larger than 0',
            ],
            numberAndZeroRules: [
                v => !!v || 'parameter is required',
                v => v >= 0 || 'value must be larger or equal to 0',
            ],
            anyNumberRules: [
                v => !!v || 'parameter is required',
            ],
            default_params: {'radius':'', 'rise': '', 'frequency': '', 'unit_size': '', 'offset': '0','rotation':'0','handedness': 'right'},
            helixFamily: [],
            inputInFocus: 1,
            snackbar: false,
            snackText: 'unspecified',
            modelName: '',
        }),
        watch: {
            // eslint-disable-next-line no-unused-vars
            helixFamily: {
                handler: function () {
                    if (this.liveCompute && this.valid) {
                        this.computeHelix(true)
                    }
                },
                deep: true
            },
        },
        methods: {
            computeHelix(compute) {
                this.sendHelixFamily();
                if (compute) {
                    this.computeHelixFamily()
                }
            },

            sendHelixFamily() {
                console.log('Sending new helix-family off for computation');
                this.$emit('updateHelixFamily', [...this.helixFamily])
            },

            computeHelixFamily() {
                this.$emit('computeHelixFamily')
            },

            newHelix() {
                // add a new helix to the family (will expose new form options)
                this.helixFamily.push(this.default_params)
            },

            setInFocus(helix_number) {
                if (this.inputInFocus === helix_number) {
                    this.inputInFocus = -1;  // set to nonexistent helix (i.e. collapse all)
                } else {
                    this.inputInFocus = helix_number;
                }
            },

            parseQueryString() {
                console.log('Checking query string');
                let searchParams =  new URLSearchParams( this.$route.hash );

                let newHelixFamily = [];
                let helix = {};
                let displayParams = {};
                const validKeys = Object.keys(this.default_params);
                let badKeys = [];

                for (let [key, value] of searchParams) {
                    key = key.replace('#','');
                    console.log(key, value)

                    if (key === 'radius') {  // we assume that the previous helix was specified if we add new radius
                        newHelixFamily.push({...this.default_params});  // add a new helix object
                        helix = newHelixFamily[newHelixFamily.length - 1];  // get the last helix object from family
                    }

                    // add value to helix if it is a valid key (specified in default_params)
                    if (validKeys.includes(key)) {
                        helix[key] = value
                    } else if (key === 'name') {
                        this.$emit('updateName', value);  // this query model has a name. Update parent.
                    } else if (key === 'n') {
                        displayParams[key] = value
                    } else if (key === 'm') {
                        displayParams[key] = value
                    } else if (key === 's') {
                        displayParams[key] = value
                    } else {
                        badKeys.push(key)
                    }
                }

                //show bad keys in snackbar
                if (badKeys.length > 0) {
                    this.snackbar = true;
                    this.snackText = 'The following invalid keys were ignored: ' + badKeys.join(', ');
                }

                if (newHelixFamily.length > 0) {
                    // we are done parsing, let's send it over for calculation
                    console.log(`query string loaded for: ${newHelixFamily.length} helix objects`);
                    this.helixFamily = newHelixFamily;
                    this.computeHelix(true);  // update values, and immediately show diffraction pattern
                    if (displayParams) {  // if not empty
                        this.$emit('newDisplayParams', displayParams);  // we have new diffraction panel display param
                    }
                } else {
                    console.log('No query string parsed.')
                }
            },

            copyHelixParams(index) {
                console.log(`Copying parameters of helix ${index + 1} into a new helix`);
                this.helixFamily.push({...this.helixFamily[index]});
                this.snackText = `Copied helix parameters from helix ${index + 1}`;
                this.snackbar = true;
            },

            toggleHandedness(helix) {
                if (helix['handedness'] === 'right') {
                    helix['handedness'] = 'left';
                } else if (helix['handedness'] === 'left') {
                    helix['handedness'] = 'right';
                }
            },

            generateModelURL(name, displayParams) {
                let exportString = window.location.href.split('#')[0];  // get URL minus any pre-existing params
                exportString += '#/#';

                // add the model name to the URL (can be empty)
                if (name) {
                    exportString += 'name=' + name.replace(/ /g, '%20')  // replace spaces in the name
                }
                // add the display parameters to the URL (can be empty)
                if (Object.entries(displayParams).length > 0) {  // check if empty
                    for (const [key, value] of Object.entries(displayParams)) {
                        if (exportString.slice(-1) !== '?') {  // prevent problems with first entry
                            exportString += '&' + key + '=' + value
                        } else { // if it is the first entry of the params, it shouldnt have an &
                            exportString += key + '=' + value
                        }
                    }
                }

                // add the helices in the URL
                for (let helix of this.helixFamily) {
                    for (const [key, value] of Object.entries(helix)) {
                        if (exportString.length > 1) {  // prevent problems with first entry
                            exportString += '&' + key + '=' + value
                        } else { // if it is the first entry of the params, it shouldnt have an &
                            exportString += key + '=' + value
                        }
                    }
                }

                return exportString
            },

            exportModel( name, displayParams ) {
                const exportString = this.generateModelURL (name, displayParams);

                // copy to user clipboard
                var dummy = document.createElement("textarea");
                document.body.appendChild(dummy);
                dummy.value = exportString;
                dummy.select();
                document.execCommand("copy");
                document.body.removeChild(dummy);
                console.log(exportString);

                this.snackText = 'copied model URL to clipboard';
                this.snackbar = true;
            },

            emitModelURL( name, displayParams ){
                const exportString = this.generateModelURL (name, displayParams);
                this.$emit('newModelURL', exportString)
            },

            // remove a single helix or all helices, and if needed, set back to the default params.
            // we assume our other logic in component ensures we cannot call this function if helixfamily.length == 1
            resetParameters( index ) {
                if (index < 0) { // then clear ALL helices
                    this.helixFamily = [this.default_params];
                } else if (this.helixFamily.length == 1) { //only a single helix in family, just reset values but do not remove
                    this.helixFamily = [this.default_params];
                } else  { // then clear a single helix at index
                    this.helixFamily.splice(index, 1);

                    // also make sure the panels are now all collapsed (otherwise it's hard to see if it has worked)
                    this.inputInFocus = -1;  // set to nonexistent helix (i.e. collapse all)
                }
                // update the other components on this new helixfamily
                this.sendHelixFamily();
            },

            test5 (arg) {
                console.log(arg)
            }
        },
        mounted() {
            this.helixFamily.push(this.default_params);
            this.parseQueryString();
        }
    }
</script>

<style scoped>
    .button-box{
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .helix-number{
        color: var(--primary);
        font-weight: bold;
        margin-bottom: 0.3rem;
        display: flex;
        justify-content: space-between;
        vertical-align: middle;
    }

    .helix-button-box{
       display: flex;
    }

    v-icon.helix-button-box{
        display: flex;
    }

    .helix-title{
        display: flex;
        justify-content: space-between;
    }

    .helix-header {
        display: flex;
    }

    .helix-number{
        width: auto;
        flex-grow: 1
    }

    .helix-button-box-focus{
        margin-left: 0.5rem;
        display: flex;
    }

    .copy-button, .delete-button{
        margin-left: 0.5rem;
    }

    .mini-field{
        width: 48%;
    }

    .optional-params{
        display: flex;
        justify-content: space-between;
    }

    .live-button{
        margin-right: 0.5rem;
    }
    .fade-enter-active {
        transition: opacity 0.2s;
    }
    .fade-enter /* .fade-leave-active below version 2.1.8 */ {
        opacity: 0;
    }

    .unit-append{
        padding: 0.2rem;
    }
</style>
