source("common.R")

# Function to calculate relative error metrics
calc_metrics <- function(estimates, card) {
  rel_err <- (estimates / card) - 1
  list(
    bias = mean(rel_err),
    rmse = sqrt(mean(rel_err^2)),
    std_dev = sd(rel_err)
  )
}

# Configuration
card <- 500000
mult <- 100
precisions <- c(4, 8, 12, 16)
iters <- 100

# Available Gumbel Transforms (from Rust code)
transforms <- c("ICDF", "BitHack")

# GHLL Algorithms that use the GumbelTransform trait
ghll_algs <- list(
  c("GHLLGeo", "GHLL Geo"),
  c("GHLLHar", "GHLL Har"),
  c("GHLLPlus", "GHLL+"),
  c("GHLLRealGeo", "GHLL Real Geo"),
  c("GHLLRealHar", "GHLL Real Har")
)

for (prec in precisions) {
  all_estimates <- list()
  all_names <- c()
  all_colors <- c()
  
  cat(sprintf("\n--- Precision k = 2^%d ---\n", prec))
  cat(sprintf("%-25s | %10s | %10s | %10s\n", "Algorithm", "Bias", "RMSE", "SD"))
  cat(paste0(rep("-", 62), collapse=""), "\n")

  # 1. Handle HLL Baseline (HLL does not use GumbelTransform)
  tryCatch({
    hll_data <- read.data("HLL", prec, card, mult)
    all_estimates <- c(all_estimates, list(hll_data / card))
    all_names <- c(all_names, "HyperLogLog")
    all_colors <- c(all_colors, "grey70")
    
    m <- calc_metrics(hll_data, card)
    cat(sprintf("%-25s | %10.6f | %10.6f | %10.6f\n", "HLL", m$bias, m$rmse, m$std_dev))
  }, error = function(e) {
    # HLL data might not be present for all configs
  })

  # 2. Handle GHLL Algorithms with different transforms
  base_colors <- rainbow(length(ghll_algs))

  for (i in 1:length(ghll_algs)) {
    alg_info <- ghll_algs[[i]]
    alg_file_name <- alg_info[1]
    alg_display_name <- alg_info[2]

    for (t in transforms) {
      full_alg_name <- paste0(alg_file_name, "_", t)
      tryCatch({
        data <- read.data(full_alg_name, prec, card, mult)
        all_estimates <- c(all_estimates, list(data / card))
        all_names <- c(all_names, paste0(alg_display_name, " (", t, ")"))
        
        # Color logic: same base color for same algorithm, darker for BitHack
        col <- base_colors[i]
        if (t == "BitHack") {
          rgb_col <- col2rgb(col) / 255
          col <- rgb(rgb_col[1]*0.6, rgb_col[2]*0.6, rgb_col[3]*0.6)
        }
        all_colors <- c(all_colors, col)

        m <- calc_metrics(data, card)
        cat(sprintf("%-25s | %10.6f | %10.6f | %10.6f\n", full_alg_name, m$bias, m$rmse, m$std_dev))
      }, error = function(e) {
        # Silent skip if file missing
      })
    }
  }

  if (length(all_estimates) == 0) {
    cat("No data found for this precision.\n")
    next
  }

  # 3. Dynamic Y-axis Calculation
  all_vals <- unlist(all_estimates)
  y_range <- range(all_vals, na.rm = TRUE)
  # Ensure the reference line (1.0) is visible and add some padding
  y_min <- min(y_range[1], 0.98) * 0.99
  y_max <- max(y_range[2], 1.02) * 1.01

  # Generate the plot
  png_out <- paste0("ghll_comp_prec_", prec, ".png")
  png(png_out, width = 1600, height = 900)
  
  par(mar = c(12, 6, 4, 2)) # Large bottom margin for vertical labels

  boxplot(all_estimates,
    main = paste0("GHLL Accuracy Comparison (n = ", format(card, scientific = FALSE), ", k = ", 2^prec, ")"),
    names = all_names,
    col = all_colors,
    las = 2, # Vertical labels for better readability
    cex.axis = 1.2,
    cex.main = 2,
    ylab = "Estimation / True Cardinality",
    ylim = c(y_min, y_max)
  )
  
  # Reference line at 1.0
  abline(h = 1, col = "red", lwd = 2, lty = 2)
  
  # Mark means with a distinct point
  means <- sapply(all_estimates, mean)
  points(1:length(all_estimates), means, pch = 18, col = "black", cex = 2)

  dev.off()
  cat(sprintf("Created plot: %s\n", png_out))
}
