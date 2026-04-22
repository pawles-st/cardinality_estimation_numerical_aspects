source("common.R")

# datasets config

card <- 500000
mult <- 100
precisions <- c(4, 8, 12, 16)
iters <- 100

# algorithms specification

base.algs <- c(
			   "HLL" = "HyperLogLog"
)
gumbel.algs <- c(
				"GHLLGeo" = "GumbelHyperLogLog (Geo)",
				"GHLLHar" = "GumbelHyperLogLog (Har)",
#				"GHLLRealGeo" = "GumbelHyperLogLog (Real, Geo)",
#				"GHLLRealHar" = "GumbelHyperLogLog (Real, Har)",
				"GHLLPlus" = "GumbelHyperLogLog+"
)

transforms <- c("ICDF", "SimpleBitHack", "TaylorBitHack")

# Plot generation

for (prec in precisions) {
	for (trans in transforms) {
		algs <- c(names(base.algs), paste0(names(gumbel.algs), "_", trans))
		algs.readable <- c(unname(base.algs), paste0(unname(gumbel.algs), "(", trans , ")"))
		no.algs <- length(algs)

		estimates <- list()

		# Read all estimates

		for (i in 1:no.algs) {
			alg.estimates <- read.data(algs[i], prec, card, mult)
			estimates <- c(estimates, list(alg.estimates / card))
		}

		# Output statistics

		cat(paste0("\n--- Statistics for transform: ", trans, ", precision: ", prec, " ---\n"))
		for (i in 1:no.algs) {
			m <- mean(estimates[[i]])
			med <- median(estimates[[i]])
			iqr_val <- IQR(estimates[[i]])
			cat(sprintf("%-20s | Mean: %0.6f | Median: %0.6f | IQR: %0.6f\n", 
						algs[i], m, med, iqr_val))
		}

		# Create a comparison boxplot

		png(paste0("boxplot_", trans, "_", prec, ".png"), width = 1920, height = 1080)
		
		par(mar = c(12, 6, 4, 2))

		boxplot(estimates,
			main = paste0("Wykres pudełkowy dla algorytmów (n = ", format(card, scientific = FALSE), ", k = ", 2^prec , ", transformata = ", trans, ")"),
			xlab = "",
			ylab = "",
			names = NA,
			col = rainbow(no.algs),
			cex.main = 3,
			cex.axis = 1.5,
			xaxt = "n"
		)
		
		# Draw x-axis without labels
		axis(1, at = 1:no.algs, labels = FALSE)

		# Add labels
		axis.labels <- c(names(base.algs), names(gumbel.algs))
		text(x = 1:no.algs, 
			 y = par("usr")[3] - (par("usr")[4] - par("usr")[3]) * 0.03, 
			 labels = axis.labels, 
			 srt = 45, 
			 adj = 1, 
			 xpd = TRUE, 
			 cex = 1.5)
		
		title(xlab = "Algorytm", line = 10, cex.lab = 3)
		title(ylab = "Estymacja / Liczba unikalnych elementów", line = 3.5, cex.lab = 3)

		# Mark the means

		means <- lapply(estimates, mean)
		points(unlist(means), pch = 3, cex = 2, lwd = 2)

		# Mark the cardinality with a line

		abline(h = 1, lwd = 2)

		# Add a legend

		legend("topleft",
			   legend = algs.readable,
			   fill = rainbow(no.algs),
			   bty = "n",
			   cex = 1.5,
			   bg = "white",
			   box.lwd = 1,
		)

		dev.off()
	}
}
