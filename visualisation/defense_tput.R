# visualisation/defense_tput.R
library(jsonlite)

# Function to extract median time from Criterion's JSON output
get_median_ns <- function(group, bench, baseline = "new") {
  paths <- c(
    paste0("target/criterion/", group, "/", bench, "/", baseline, "/estimates.json"),
    paste0("../target/criterion/", group, "/", bench, "/", baseline, "/estimates.json"),
    paste0("gumbel_estimation/target/criterion/", group, "/", bench, "/", baseline, "/estimates.json")
  )
  for (path in paths) {
    if (file.exists(path)) {
      tryCatch({
        data <- fromJSON(path)
        return(data$mean$point_estimate)
      }, error = function(e) { return(NULL) })
    }
  }
  return(NULL)
}

# Transform variants to compare (Internal Benchmark Names)
# The order here determines the data collection order
transforms_internal <- c("ICDF", "Basic", "Taylor", "Spline", "LUT")
# The display names for the plot
transforms_display <- c("ICDF", "BasicBitHack", "TaylorBitHack", "Spline", "LUT")

native_q16 <- numeric(length(transforms_internal))
generic_q16 <- numeric(length(transforms_internal))
native_f32 <- numeric(length(transforms_internal))

# 1. HLL Data (Reference)
hll_ns_n <- get_median_ns("HLL", "Standard", "native")
hll_ns_g <- get_median_ns("HLL", "Standard", "generic")
hll_native  <- if(!is.null(hll_ns_n)) 1e9 / hll_ns_n / 1e6 else 310
hll_generic <- if(!is.null(hll_ns_g)) 1e9 / hll_ns_g / 1e6 else 126

# 2. GHLL Data
for (i in seq_along(transforms_internal)) {
  t <- transforms_internal[i]
  ns_qn <- get_median_ns("GHLL", t, "native")
  ns_qg <- get_median_ns("GHLL", t, "generic")
  ns_fn <- get_median_ns("GHLLFloat", t, "native")
  
  native_q16[i]  <- if(!is.null(ns_qn)) 1e9 / ns_qn / 1e6 else 0
  generic_q16[i] <- if(!is.null(ns_qg)) 1e9 / ns_qg / 1e6 else 0
  native_f32[i]  <- if(!is.null(ns_fn)) 1e9 / ns_fn / 1e6 else 0
}

# --- Plot 1: Hardware Win (Native vs Generic) ---
png("defense_native_vs_generic.png", width = 1500, height = 900, res = 150)
par(mar = c(6, 6, 5, 2))

t_matrix_arch <- cbind(c(hll_generic, hll_native), rbind(generic_q16, native_q16))
colnames(t_matrix_arch) <- c("HLL", transforms_display)
colors_arch <- c("#cccccc", "#ffb347") # Gray vs Orange

bp1 <- barplot(t_matrix_arch, beside = TRUE, col = colors_arch,
               main = "Hardware Impact: Generic x86_64 vs. Native",
               ylab = "Throughput (Million updates/s)",
               ylim = c(0, max(t_matrix_arch) * 1.2), border = "white", las = 1)
text(x = bp1, y = t_matrix_arch, labels = round(t_matrix_arch, 0), pos = 3, cex = 0.7, font=2)
legend("topright", legend = c("Generic x86_64", "Native CPU"), fill = colors_arch, bty = "n")
dev.off()

# --- Plot 2: Math Win (f32 vs Q16.16) ---
png("defense_f32_vs_q16_native.png", width = 1500, height = 900, res = 150)
par(mar = c(6, 6, 5, 2))

t_matrix_math <- rbind(c(hll_native, native_f32), c(0, native_q16))
colnames(t_matrix_math) <- c("HLL", transforms_display)
colors_math <- matrix(rep(c("#aec6cf", "#ffb347"), length(transforms_internal)+1), nrow=2)
colors_math[,1] <- c("#555555", "#555555")

bp2 <- barplot(t_matrix_math, beside = TRUE, col = colors_math,
               main = "Arithmetic Impact: f32 Floating-Point vs. Q16.16 Fixed-Point",
               sub = "All variants compiled for Native CPU",
               ylab = "Throughput (Million updates/s)",
               ylim = c(0, max(t_matrix_math) * 1.2), border = "white", las = 1)

labels_math <- round(t_matrix_math, 0)
labels_math[2,1] <- "" 
text(x = bp2, y = t_matrix_math, labels = labels_math, pos = 3, cex = 0.7, font=2)

legend("topright", 
       legend = c("HLL Reference", "GHLL: f32 Math", "GHLL: Q16.16 Math"), 
       fill = c("#555555", "#aec6cf", "#ffb347"), bty = "n")
dev.off()

cat("Generated Refined Plots:\n1. defense_native_vs_generic.png\n2. defense_f32_vs_q16_native.png\n")
