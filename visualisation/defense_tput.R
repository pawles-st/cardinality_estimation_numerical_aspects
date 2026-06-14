# visualisation/defense_tput.R
library(jsonlite)

# Function to extract median time from Criterion's JSON output
get_median_ns <- function(group, bench, baseline = "new") {
  path <- paste0("target/criterion/", group, "/", bench, "/", baseline, "/estimates.json")
  if (file.exists(path)) {
    tryCatch({
      data <- fromJSON(path)
      val <- data$mean$point_estimate
      cat(sprintf("Loaded %s/%s [%s]: %.2f ns\n", group, bench, baseline, val))
      return(val)
    }, error = function(e) { return(NULL) })
  }
  return(NULL)
}

# Transform variants to compare (Internal Benchmark Names)
transforms_internal <- c("Naive", "Pade", "BasicBitHack", "CorrectedBitHack", "Spline", "LUT")
transforms_display <- c("Naive", "Pade", "BasicBitHack", "CorrectedBitHack", "Spline", "LUT")

native_q16 <- numeric(length(transforms_internal))
generic_q16 <- numeric(length(transforms_internal))
native_f32 <- numeric(length(transforms_internal))

# 1. HLL Data
ns_hn <- get_median_ns("HLL", "Standard", "native")
ns_hg <- get_median_ns("HLL", "Standard", "generic")

hll_native  <- if(!is.null(ns_hn)) 1e9 / ns_hn / 1e6 else 0
hll_generic <- if(!is.null(ns_hg)) 1e9 / ns_hg / 1e6 else 0

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
png("visualisation/defense_native_vs_generic.png", width = 1500, height = 900, res = 150)
par(mar = c(8, 6, 5, 2))

t_matrix_arch <- cbind(c(hll_generic, hll_native), rbind(generic_q16, native_q16))
colnames(t_matrix_arch) <- c("HLL", transforms_display)
colors_arch <- c("#cccccc", "#ffb347") # Gray vs Orange

bp1 <- barplot(t_matrix_arch, beside = TRUE, col = colors_arch,
               main = "Wpływ architektury: ogólna x86_64 vs. natywna (GHLL+)",
               ylab = "Przepustowość (mln aktualizacji/s)",
               ylim = c(0, max(t_matrix_arch) * 1.2), border = "white", las = 1, 
               cex.main = 1.4, cex.lab = 1.2, xaxt = "n")

abline(h = 0)
text(x = colMeans(bp1), y = -max(t_matrix_arch)*0.05, labels = colnames(t_matrix_arch), 
     srt = 45, adj = 1, xpd = TRUE, cex = 1.1)

text(x = bp1, y = t_matrix_arch, labels = round(t_matrix_arch, 0), pos = 3, cex = 0.9, font=2)
legend("topright", legend = c("Ogólna x86_64", "Natywna CPU"), fill = colors_arch, bty = "n", cex = 1.1)
dev.off()

# --- Plot 2: Math Win (f32 vs Q16.16) ---
png("visualisation/defense_f32_vs_q16_native.png", width = 1500, height = 900, res = 150)
par(mar = c(8, 6, 5, 2))

t_matrix_math <- rbind(c(hll_native, native_f32), c(0, native_q16))
colnames(t_matrix_math) <- c("HLL", transforms_display)
colors_math <- matrix(rep(c("#aec6cf", "#ffb347"), length(transforms_internal)+1), nrow=2)
colors_math[,1] <- c("#555555", "#555555")

bp2 <- barplot(t_matrix_math, beside = TRUE, col = colors_math,
               main = "Wpływ arytmetyki: f32 vs. Q16.16 (GHLL+ Native)",
               ylab = "Przepustowość (mln aktualizacji/s)",
               ylim = c(0, max(t_matrix_math) * 1.2), border = "white", las = 1, 
               cex.main = 1.4, cex.lab = 1.2, xaxt = "n")

abline(h = 0)
text(x = colMeans(bp2), y = -max(t_matrix_math)*0.05, labels = colnames(t_matrix_math), 
     srt = 45, adj = 1, xpd = TRUE, cex = 1.1)

labels_math <- round(t_matrix_math, 0)
labels_math[2,1] <- "" 
text(x = bp2, y = t_matrix_math, labels = labels_math, pos = 3, cex = 0.9, font=2)

legend("topright", 
       legend = c("HLL (referencyjny)", "GHLL+: f32", "GHLL+: Q16.16"), 
       fill = c("#555555", "#aec6cf", "#ffb347"), bty = "n", cex = 1.1)
dev.off()

cat("Generated Final Verified Plots:\n1. defense_native_vs_generic.png\n2. defense_f32_vs_q16_native.png\n")
