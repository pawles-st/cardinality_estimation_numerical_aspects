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

transforms <- c("ICDF", "BitHack")

# plot generation

for (trans in transforms) {
	algs <- c(names(base.algs), paste0(names(gumbel.algs), "_", trans))
	algs.readable <- c(unname(base.algs), paste0(unname(gumbel.algs), "(", trans , ")"))
	no.algs <- length(algs)

	for (prec in precisions) {

		estimates <- list()

		# read all estimates

		for (i in 1:no.algs) {
			alg.estimates <- read.data(algs[i], prec, card, mult)
			estimates <- c(estimates, list(alg.estimates / card))
		}

		# create a comparison boxplot

		png(paste0("boxplot_", trans, "_", prec, ".png"), width = 1920, height = 1080)
		
		par(mar = c(12, 6, 4, 2))

		boxplot(estimates,
			main = paste0("Wykres pudełkowy dla algorytmów (n = ", format(card, scientific = FALSE), ", k = ", 2^prec , ", transformata = ", trans, ")"),
			xlab = "",
			ylab = "",
			names = algs,
			col = rainbow(no.algs),
			cex.main = 3,
			cex.axis = 1.5,
			las = 2
		)
		
		title(xlab = "Algorytm", line = 10, cex.lab = 3)
		title(ylab = "Estymacja / Liczba unikalnych elementów", line = 3.5, cex.lab = 3)

		# mark the means

		means <- lapply(estimates, mean)
		points(unlist(means), pch = 3, cex = 2, lwd = 2)

		# mark the cardinality with a line

		abline(h = 1, lwd = 2)

		# add a legend

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
