import * as d3 from "d3";

export function  draw_overlay (svg_el, wasm_bessel_max, helix, m_order, n_order_list, scale_factor ) {
    const colors = ['white',  '#FF5d5d', '#FA8072','#ffc013','#ffdb79', '#1f66e5', '#6495ED', '#505050', '#B8B8B8'];
    let m_vals = [];
    for (let i = 0; i < m_order+1; i++) {
        m_vals.push(i);
        if (i!==0) {
            m_vals.push(-i);
        }
    }

    // we also add a textbox that will be shown when hovered. We could keep this (not delete it every time)
    // but it is not significant enough (performance wise) to worry about.
    let tooltip = d3.select("body").append("div")
        .attr("class", "maxima maxima-tooltip")
        .style("opacity", 0);

    let color_index = 0;
    for (const m of m_vals) {
        //console.log(`m is ${m}`);
        let data = [];
        for (const n in n_order_list) {
            const z_center = - m / (helix.rise * scale_factor);  // center to draw +- z_val from
            const z_val = n / (helix.frequency * helix.rise * scale_factor);
            const r_val = wasm_bessel_max(+n) / (scale_factor * 2 * Math.PI * helix.radius);
            data.push({'z': z_center + z_val, 'r': r_val, 'n':n,'m':m});
            if (z_val !== 0) {
                data.push({'z': z_center - z_val, 'r': r_val, 'n': n, 'm': m});
            }
            if (r_val !== 0) {
                data.push({'z': z_center + z_val, 'r': -r_val, 'n': n, 'm': m});
                data.push({'z': z_center - z_val, 'r': -r_val, 'n': n, 'm': m});
            }
        }
        svg_el.selectAll("maxima")
            .data(data)
            .enter().append("circle")
            .attr('class', `maxima m_num_${m}`)
            .attr("r", 4)
            .attr("cx", function (d) {
                return d.r;
            })
            .attr("cy", function (d) {
                return d.z;
            })
            .attr("stroke", "black")
            .attr("stroke-width", 1.5)
            .attr("fill", colors[color_index])
            .on('mouseover', function (event, d) {
                // make point grow
                d3.select(this).transition()
                    .duration('100')
                    .attr("r", 7);
                // show tooltip
                tooltip.transition()
                    .duration(100)
                    .style("opacity", 1);
                // set location and text tooltip
                tooltip.html(`n = ${d.n} | m = ${d.m}`)
                    .style("left", (event.pageX  + 10) + "px")
                    .style("top", (event.pageY - 15) + "px");
                // fade all the dots (we will un-fade the ones in focus below explicity)
                document.querySelectorAll('.maxima').forEach((el) => el.style.opacity = 0.3);
                // now un-fade those elements that are of the same [m]
                document.querySelectorAll(`.m_num_${m}`).forEach((el) => el.style.opacity = 1);
            })
            .on('mouseout', function (d, i) {
                // shrink point back to regular size
                d3.select(this).transition()
                    .duration('200')
                    .attr("r", 4);
                // hide tooltip
                tooltip.transition()
                    .duration('200')
                    .style("opacity", 0);
                document.querySelectorAll('.maxima').forEach((el) => el.style.opacity = 1)
            });
        color_index++
    }
}
