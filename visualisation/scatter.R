source("visualisation/common.R")

# datasets config
cardinalities <- seq(from = 10000, to = 600000, by = 10000)
mult <- 100
precisions <- c(4, 8, 12, 16)
iters <- 100

# algorithms specification
ghll.variant <- "GHLLPlus"
base.algs <- c("HLL" = "HyperLogLog")
gumbel.algs <- c("GHLLPlus" = "GumbelHyperLogLog")
transforms <- c("Naive", "BasicBitHack", "CorrectedBitHack", "LUT", "Pade", "Spline")

# All algorithms to plot separately
all_algs <- c(names(base.algs))
for (trans in transforms) {
    all_algs <- c(all_algs, paste0(ghll.variant, "_", trans))
}

# Plot generation
for (prec in precisions) {
    for (alg in all_algs) {
        
        # Determine readable name and color
        alg_col <- "#27ae60" # Default Green
        if (alg == "HLL") {
            alg_name <- "HyperLogLog"
            alg_col <- "#555555"
        } else {
            trans_part <- sub(paste0(ghll.variant, "_"), "", alg)
            alg_name <- trans_part
            # Map to triadic colors
            if (trans_part == "Naive") alg_col <- "#27ae60"
            if (trans_part == "Pade") alg_col <- "#2ecc71"
            if (trans_part == "BasicBitHack") alg_col <- "#9b59b6"
            if (trans_part == "CorrectedBitHack") alg_col <- "#8e44ad"
            if (trans_part == "Spline") alg_col <- "#f39c12"
            if (trans_part == "LUT") alg_col <- "#e67e22"
        }

        estimates <- list()
        valid_cards <- c()

        # Read estimates for each cardinality
        for (card in cardinalities) {
            # Check if file exists before reading
            data.text <- paste(alg, prec, format(card, scientific = FALSE), format(card * mult, scientific = FALSE), sep = '_')
            filename <- paste0("results/", data.text, ".txt")
            
            if (file.exists(filename)) {
                alg.estimates <- read.data(alg, prec, card, mult)
                estimates <- c(estimates, list(alg.estimates / card))
                valid_cards <- c(valid_cards, card)
            }
        }

        if (length(estimates) == 0) next

        # Create the plot
        png(paste0("visualisation/scatter_n_trend_", alg, "_", prec, ".png"), width = 1200, height = 900, res = 120)
        par(mar = c(6, 7, 5, 2)) # Increased left and bottom margins

        # Flatten estimates for plotting
        y_vals <- unlist(estimates)
        x_vals <- rep(valid_cards, each = iters)

        # Calculate limits for better visibility
        y_min <- min(y_vals)
        y_max <- max(y_vals)
        
        # Determine fixed limits or dynamic ones
        y_lim_min <- min(1.0, max(0.5, y_min - 0.05))
        y_lim_max <- max(1.0, min(1.5, y_max + 0.05))

        plot(x = x_vals,
             y = y_vals,
             main = paste0(alg_name, " (k = 2^", prec, ")"),
             xlab = "Liczba unikalnych elementów (n)",
             ylab = "Estymacja / Liczba unikalnych elementów",
             ylim = c(y_lim_min, y_lim_max),
             col = adjustcolor(alg_col, alpha.f = 0.2), 
             pch = 16,
             cex = 0.8,
             cex.main = 2,
             cex.axis = 1.5,
             cex.lab = 1.8,
             las = 1,
             mgp = c(4, 1, 0) # Adjust title/label placement to avoid overlap
        )

        # Mark the means for each n
        means <- sapply(estimates, mean)
        lines(x = valid_cards, y = means, col = "red", lwd = 4)
        points(x = valid_cards, y = means, col = "red", pch = 18, cex = 1.5)

        # Ideal result line
        abline(h = 1, lwd = 2, lty = 2, col = "black")

        # Add a frame (box) around the plot
        box(lwd = 2)

        dev.off()
        cat(paste0("Generated n-trend scatterplot: scatter_n_trend_", alg, "_", prec, ".png\n"))
    }
}
